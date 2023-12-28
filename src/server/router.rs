use crate::context::ServerContext;
use crate::service;
use http::{HeaderValue, Method, Request, Response, StatusCode};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::service::Service;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct Router {
    context: Arc<ServerContext>,
    remote_address: SocketAddr,
}

impl Router {
    pub fn new(context: Arc<ServerContext>, remote_address: SocketAddr) -> Self {
        Self {
            context,
            remote_address,
        }
    }

    pub async fn route(
        self,
        request: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, http::Error> {
        let ignore_body = request.method() == Method::HEAD;
        let result = match (request.method(), request.uri().path()) {
            (_, path) if path.starts_with("/cgi-bin/") => self.invoke_cgi(request).await,
            (method, path) => self.load_file(method, path).await,
        };

        match result {
            Ok(mut response) => {
                if ignore_body {
                    *response.body_mut() = Full::default();
                }

                Ok(response)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::default()),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(e.to_string())),
        }
    }

    async fn invoke_cgi(&self, request: Request<Incoming>) -> io::Result<Response<Full<Bytes>>> {
        let handler = service::CommonGatewayInterface::new(
            self.context.clone(),
            self.remote_address,
            request,
        );

        let response = handler.serve().await?;

        match response.status() {
            code if code.is_success()
                && !response.headers().contains_key(http::header::CONTENT_TYPE) =>
            {
                Err(io::Error::from(io::ErrorKind::Unsupported))
            }
            code if code.is_redirection() && response.headers().len() > 1 => {
                Err(io::Error::from(io::ErrorKind::Unsupported))
            }
            _ => Ok(response),
        }
    }

    async fn load_file(&self, method: &Method, path: &str) -> io::Result<Response<Full<Bytes>>> {
        let file_path = self.context.resolve_path(path);

        if file_path.extension() == Some("cgi".as_ref()) {
            let mut response = Response::new(Full::default());
            *response.status_mut() = StatusCode::FORBIDDEN;
            return Ok(response);
        }

        match method {
            &Method::HEAD => {
                let file = File::open(file_path).await?;
                let metadata = file.metadata().await?;
                let mut response = Response::new(Full::default());

                response.headers_mut().insert(
                    http::header::CONTENT_LENGTH,
                    HeaderValue::from(metadata.len()),
                );

                Ok(response)
            }
            &Method::GET => {
                let mut file = File::open(file_path).await?;

                let metadata = file.metadata().await?;
                let length = usize::try_from(metadata.len()).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::OutOfMemory,
                        "Unable to buffer file contents.",
                    )
                })?;

                let mut buffer = Vec::with_capacity(length);

                file.read_to_end(&mut buffer).await?;

                let mut response = Response::new(Full::from(buffer));

                response.headers_mut().insert(
                    http::header::CONTENT_LENGTH,
                    HeaderValue::from(metadata.len()),
                );

                Ok(response)
            }
            _ => {
                let mut response = Response::new(Full::default());
                *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
                return Ok(response);
            }
        }
    }
}

impl Service<Request<Incoming>> for Router {
    type Response = Response<Full<Bytes>>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        Box::pin(self.clone().route(request))
    }
}
