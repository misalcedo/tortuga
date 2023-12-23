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
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::{pin, select};

const MAX_BODY_BYTES: u64 = 1024 * 64;

pub struct NonParsedHeader {
    context: Arc<ServerContext>,
    remote_address: SocketAddr,
}

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
    pub async fn serve(&self, request: Request<Incoming>) -> io::Result<Response<Full<Bytes>>> {
        let mut child = Command::new(self.context.script_filename())
            .kill_on_drop(true)
            .env_clear()
            .env("PATH", self.context.path())
            .env("SERVER_SOFTWARE", self.context.software())
            .env("GATEWAY_INTERFACE", "CGI/1.1")
            .env("SERVER_PROTOCOL", format!("{:?}", request.version()))
            .env("SCRIPT_FILENAME", self.context.script_filename())
            .env("SCRIPT_NAME", self.context.script_name())
            .env("SERVER_ADDR", self.context.ip_address())
            .env("SERVER_PORT", self.context.port())
            .env("REMOTE_ADDR", self.remote_address.ip().to_string())
            .env("REMOTE_PORT", self.remote_address.port().to_string())
            .env("PATH_INFO", request.uri().path())
            .env("REQUEST_METHOD", request.method().as_str())
            .envs(request.uri().query().map(|q| ("QUERY_STRING", q)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut stdin = child.stdin.take().map(BufWriter::new);
        let mut stdout = child.stdout.take().map(BufReader::new);

        let body = request.into_body();

        // Protect our server from massive bodies.
        let upper = body.size_hint().upper().unwrap_or(u64::MAX);
        if upper > MAX_BODY_BYTES {
            let mut response = Response::new(Full::new(Bytes::from(format!(
                "Body size of {upper} bytes is too large. The largest supported body is {MAX_BODY_BYTES}"
            ))));

            *response.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;

            return Ok(response);
        }

        let whole_body = body
            .collect()
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?
            .to_bytes();
        let reader_task = async {
            if let Some(mut stdin) = stdin.as_mut() {
                stdin.write_all(whole_body.as_ref()).await
            } else {
                Ok(())
            }
        };

        let writer_task = async {
            if let Some(mut stdout) = stdout.as_mut() {
                Err(io::Error::from(io::ErrorKind::InvalidData))
                //tokio::io::copy(&mut stdout, &mut writer).await
            } else {
                Ok(0)
            }
        };
        pin!(reader_task);
        pin!(writer_task);

        let mut reader_done = false;
        let mut writer_done = false;

        loop {
            select! {
                _ = child.wait() => {
                    //eprintln!("Child exited with status {:?}.", output.status);
                    //eprintln!("{}", String::from_utf8_lossy(output.stderr.as_slice()));

                    return Ok(Response::new(Full::new(Bytes::from("TODO"))));
                }
                _ = &mut reader_task, if !reader_done => {
                    reader_done = true;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    child.kill().await?;

                    let mut response = Response::new(Full::new(Bytes::from("Request timed out.")));

                    *response.status_mut() = StatusCode::GATEWAY_TIMEOUT;

                    return Ok(response);
                }
            }
        }
    }
}

impl Service<Request<Incoming>> for NonParsedHeader {
    type Response = Response<Full<Bytes>>;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        Box::pin(self.serve(request))
    }
}
