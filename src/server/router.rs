use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use http::{Method, Request, Response, StatusCode};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::service::Service;
use crate::context::ServerContext;
use crate::service;

pub struct Router {
    context: Arc<ServerContext>,
    remote_address: SocketAddr
}

impl Router {
    pub fn new(context: Arc<ServerContext>, remote_address: SocketAddr) -> Self {
        Self {
            context,
            remote_address
        }
    }
}

impl Service<Request<Incoming>> for Router {
    type Response = Response<Full<Bytes>>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        let handler =
            service::CommonGatewayInterface::new(self.context.clone(), self.remote_address, request);

        Box::pin(async {
            match handler.serve().await {
                Ok(response) => Ok(response),
                Err(e) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::from(e.to_string()))
            }
        })

        // match (request.method(), request.uri().path()) {
        //     (&method, path) if path.starts_with("/cgi-bin/") => {
        //         // TODO: invoke CGI script.
        //         Ok(Response::new(Full::from(
        //             "",
        //         )))
        //     },
        //     (&Method::GET, _path) => {
        //         // TODO: serve static content from document root.
        //         Ok(Response::new(Full::from(
        //             "",
        //         )))
        //     },
        //     _ => {
        //         Response::builder()
        //             .status(StatusCode::NOT_FOUND)
        //             .body(Full::from(""))
        //     }
        // }
    }
}