use crate::context::ServerContext;
use crate::service;
use http::{HeaderValue, Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Collected, Full};
use hyper::body::{Body, Bytes, Incoming};
use hyper::service::Service;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const MAX_BODY_BYTES: u64 = 1024 * 64;

trait CgiResponse {
    fn is_document(&self) -> bool;
    fn is_local_redirect(&self) -> bool;
    fn is_client_redirect(&self) -> bool;
    fn is_client_redirect_with_document(&self) -> bool;
}

impl CgiResponse for Response<Full<Bytes>> {
    fn is_document(&self) -> bool {
        (self.status().is_success() || self.status().is_client_error())
            && self.headers().contains_key(http::header::CONTENT_TYPE)
    }

    fn is_local_redirect(&self) -> bool {
        self.status() == StatusCode::OK
            && self.body().size_hint().lower() == 0
            && self.body().size_hint().exact() == Some(0)
            && self.headers().len() == 1
            && self
                .headers()
                .get(http::header::LOCATION)
                .map(|l| {
                    // Local URI's must either have an empty path and query, have a non-empty path or have a non-empty query.
                    l.is_empty() || l.as_bytes().starts_with(b"/") || l.as_bytes().starts_with(b"?")
                })
                .unwrap_or(false)
    }

    fn is_client_redirect(&self) -> bool {
        self.status() == StatusCode::OK
            && self.body().size_hint().lower() == 0
            && self.body().size_hint().exact() == Some(0)
            && self.headers().len() == 1
            && self
                .headers()
                .get(http::header::LOCATION)
                .map(|l| {
                    l.as_bytes().starts_with(b"http://") || l.as_bytes().starts_with(b"https://")
                })
                .unwrap_or(false)
    }

    fn is_client_redirect_with_document(&self) -> bool {
        self.status().is_redirection()
            && self.headers().contains_key(http::header::LOCATION)
            && self.headers().contains_key(http::header::CONTENT_TYPE)
    }
}

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
        let upper = request.body().size_hint().upper().unwrap_or(u64::MAX);
        if upper > MAX_BODY_BYTES {
            return Response::builder()
                .status(StatusCode::PAYLOAD_TOO_LARGE)
                .body(Full::from(format!(
                    "Body size of {upper} bytes is too large. The largest supported body is {MAX_BODY_BYTES}"
                )));
        }

        let (parts, body) = request.into_parts();
        let body = match body.collect().await.map(Collected::to_bytes) {
            Ok(b) => b,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(Full::from("Unable to read the full request body."));
            }
        };
        let request = Request::from_parts(parts, body);

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

    async fn invoke_cgi(&self, request: Request<Bytes>) -> io::Result<Response<Full<Bytes>>> {
        let handler = service::CommonGatewayInterface::new(
            self.context.clone(),
            self.remote_address,
            request,
        );

        let mut response = handler.serve().await?;

        if response.is_document() || response.is_client_redirect_with_document() {
            Ok(response)
        } else if response.is_local_redirect() {
            Err(io::Error::from(io::ErrorKind::Unsupported))
        } else if response.is_client_redirect() {
            *response.status_mut() = StatusCode::FOUND;
            Ok(response)
        } else {
            Err(io::Error::from(io::ErrorKind::Unsupported))
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
