use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::time::Duration;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::pin;

use crate::cgi;

#[derive(Debug, Clone)]
struct CgiService {
    script: PathBuf
}

impl From<PathBuf> for CgiService {
    fn from(script: PathBuf) -> Self {
        Self {
            script
        }
    }
}

impl Service<Request<Incoming>> for CgiService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, _request: Request<Incoming>) -> Self::Future {
        let response = match cgi::run(&self.script, vec![""], HashMap::from([("", "")])) {
            Ok(output) if output.status.success() => {
                Ok(Response::new(Full::new(Bytes::from(output.stdout))))
            }
            Ok(output) => {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::new(Bytes::from(output.stderr)))
            }
            Err(e) => {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::new(Bytes::from(e.to_string())))
            }
        };

        Box::pin(async { response })
    }
}

pub async fn serve(script: PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use a 5 second timeout for incoming connections to the server.
    // If a request is in progress when the 5 second timeout elapses,
    // use a 2 second timeout for processing the final request and graceful shutdown.
    let connection_timeouts = vec![Duration::from_secs(5), Duration::from_secs(2)];

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    let service = CgiService::from(script);

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Clone the connection_timeouts so they can be passed to the new task.
        let connection_timeouts_clone = connection_timeouts.clone();
        let connection_service = service.clone();

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Pin the connection object so we can use tokio::select! below.
            let conn = http1::Builder::new().serve_connection(io, connection_service);
            pin!(conn);

            // Iterate the timeouts.  Use tokio::select! to wait on the
            // result of polling the connection itself,
            // and also on tokio::time::sleep for the current timeout duration.
            for (iter, sleep_duration) in connection_timeouts_clone.iter().enumerate() {
                tokio::select! {
                    res = conn.as_mut() => {
                        // Polling the connection returned a result.
                        // In this case print either the successful or error result for the connection
                        // and break out of the loop.
                        match res {
                            Ok(()) => {},
                            Err(e) =>  eprintln!("error serving connection: {:?}", e),
                        };
                        break;
                    }
                    _ = tokio::time::sleep(*sleep_duration) => {
                        // tokio::time::sleep returned a result.
                        // Call graceful_shutdown on the connection and continue the loop.
                        eprintln!("iter = {} got timeout_interval, calling conn.graceful_shutdown", iter);
                        conn.as_mut().graceful_shutdown();
                    }
                }
            }
        });
    }
}
