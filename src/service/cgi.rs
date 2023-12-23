use crate::context::ServerContext;
use http_body_util::{BodyExt, Full};
use hyper::body::{Body, Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;

const MAX_BODY_BYTES: u64 = 1024 * 64;

pub struct NonParsedHeader {
    context: Arc<ServerContext>,
    remote_address: SocketAddr,
}

// TODO: rename and fix bug where the connection cannot be re-used.
impl NonParsedHeader {
    pub fn new(context: Arc<ServerContext>, remote_address: SocketAddr) -> Self {
        Self {
            context,
            remote_address,
        }
    }

    pub async fn run(self, stream: TcpStream) -> Result<(), hyper::Error> {
        http1::Builder::new()
            .serve_connection(TokioIo::new(stream), self)
            .await
    }

    pub async fn serve(
        context: Arc<ServerContext>,
        remote_address: SocketAddr,
        request: Request<Incoming>,
    ) -> io::Result<Response<Full<Bytes>>> {
        // Protect our server from massive bodies.
        let upper = request.body().size_hint().upper().unwrap_or(u64::MAX);
        if upper > MAX_BODY_BYTES {
            let mut response = Response::new(Full::new(Bytes::from(format!(
                "Body size of {upper} bytes is too large. The largest supported body is {MAX_BODY_BYTES}"
            ))));

            *response.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;

            return Ok(response);
        }

        let (request, body) = request.into_parts();
        let whole_body = body
            .collect()
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?
            .to_bytes();

        let mut child = Command::new(context.script_filename())
            .kill_on_drop(true)
            .env_clear()
            .env("PATH", context.path())
            .env("SERVER_SOFTWARE", context.software())
            .env("GATEWAY_INTERFACE", "CGI/1.1")
            .env("SERVER_PROTOCOL", format!("{:?}", request.version))
            .env("SCRIPT_FILENAME", context.script_filename())
            .env("SCRIPT_NAME", context.script_name())
            .env("SERVER_ADDR", context.ip_address())
            .env("SERVER_PORT", context.port())
            .env("REMOTE_ADDR", remote_address.ip().to_string())
            .env("REMOTE_PORT", remote_address.port().to_string())
            .env("PATH_INFO", request.uri.path())
            .env("REQUEST_METHOD", request.method.as_str())
            .envs(request.uri.query().map(|q| ("QUERY_STRING", q)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut stdin = child.stdin.take();
        let mut stdout = child.stdout.take();

        let cancel = CancellationToken::new();
        let stdin_cancel = cancel.child_token();
        let _cancel_guard = cancel.drop_guard();

        tokio::spawn(async move {
            if let Some(stdin) = stdin.as_mut() {
                select! {
                    result = stdin.write_all(whole_body.as_ref()) => { result }
                    _ = stdin_cancel.cancelled() => {
                        eprintln!("Cancelled!");
                        Ok(())
                    }
                }
            } else {
                Ok(())
            }
        });

        let stdout_task = async {
            let mut output = Vec::with_capacity(1024 * 8);

            if let Some(stdout) = stdout.as_mut() {
                stdout.read_to_end(&mut output).await?;
            }

            Ok::<Vec<u8>, io::Error>(output)
        };

        match try_join!(
            tokio::time::timeout(Duration::from_secs(1), child.wait()),
            tokio::time::timeout(Duration::from_secs(1), stdout_task),
        ) {
            Ok((Ok(status), Ok(output))) if status.success() => {
                return Ok(Response::new(Full::new(Bytes::from(output))));
            }
            Ok(_) => {
                child.kill().await?;

                let mut response =
                    Response::new(Full::new(Bytes::from("Unable to wait for child process.")));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                Ok(response)
            }
            Err(_) => {
                child.kill().await?;

                let mut response = Response::new(Full::new(Bytes::from("Request timed out.")));
                *response.status_mut() = StatusCode::GATEWAY_TIMEOUT;
                Ok(response)
            }
        }
    }
}

impl Service<Request<Incoming>> for NonParsedHeader {
    type Response = Response<Full<Bytes>>;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        Box::pin(Self::serve(
            self.context.clone(),
            self.remote_address,
            request,
        ))
    }
}
