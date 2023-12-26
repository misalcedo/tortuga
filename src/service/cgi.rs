use crate::context::ServerContext;
use crate::variable::ToMetaVariable;
use http_body_util::{BodyExt, Full};
use httparse::Status;
use hyper::body::{Body, Bytes, Incoming};
use hyper::http::{HeaderName, HeaderValue};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{HeaderMap, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::process::Stdio;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;

const MAX_BODY_BYTES: u64 = 1024 * 64;

pub struct CommonGatewayInterface {
    context: Arc<ServerContext>,
    remote_address: SocketAddr,
}

impl CommonGatewayInterface {
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
        let upper = request.body().size_hint().upper().unwrap_or(u64::MAX);
        if upper > MAX_BODY_BYTES {
            return Response::builder()
                .status(StatusCode::PAYLOAD_TOO_LARGE)
                .body(Full::new(Bytes::from(format!(
                    "Body size of {upper} bytes is too large. The largest supported body is {MAX_BODY_BYTES}"
                ))))
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidData));
        }

        let (request, body) = request.into_parts();
        let collected_body = body
            .collect()
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

        let headers = request.headers.iter().map(|(key, value)| {
            (
                key.as_str().to_meta_variable(Some("HTTP")),
                String::from_utf8_lossy(value.as_bytes()).to_string(),
            )
        });
        let trailers = collected_body
            .trailers()
            .into_iter()
            .flat_map(HeaderMap::iter)
            .map(|(key, value)| {
                (
                    key.as_str().to_meta_variable(Some("HTTP")),
                    String::from_utf8_lossy(value.as_bytes()).to_string(),
                )
            });
        let mut child = Command::new(context.script_filename())
            .kill_on_drop(true)
            .current_dir(context.working_directory())
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
            .envs(headers)
            .envs(trailers)
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
            if let Some(mut stdin) = stdin.take() {
                let mut input = collected_body.aggregate();

                select! {
                    _ = stdin.write_all_buf(&mut input) => {}
                    _ = stdin_cancel.cancelled() => {}
                }

                drop(stdin);
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
                let output = Bytes::from(output);
                let mut response = Response::new(Full::from(output.clone()));
                let mut headers = [httparse::EMPTY_HEADER; 256];
                let mut offset = 0;

                match httparse::parse_headers(&output, &mut headers) {
                    Ok(Status::Complete((last, headers))) => {
                        offset = last;

                        for header in headers {
                            match header.name {
                                "Status" => {
                                    if let Ok(status_code) = StatusCode::from_bytes(header.value) {
                                        *response.status_mut() = status_code;
                                    }
                                }
                                _ => {
                                    match (
                                        HeaderName::from_str(header.name),
                                        HeaderValue::from_bytes(header.value),
                                    ) {
                                        (Ok(name), Ok(value)) => {
                                            response.headers_mut().insert(name, value);
                                        }
                                        _ => {
                                            eprintln!("Skipping invalid header '{}'", header.name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(Status::Partial) => {
                        eprintln!("Partial headers.")
                    }
                    Err(e) => {
                        eprintln!("Encountered an error parsing headers: {e}")
                    }
                }

                if offset != 0 {
                    *response.body_mut() = Full::from(output.slice(offset..));
                }

                return Ok(response);
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

impl Service<Request<Incoming>> for CommonGatewayInterface {
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

#[cfg(test)]
mod tests {
    #[test]
    fn empty() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"\r\n";
        let result = httparse::parse_headers(input, &mut headers).unwrap();

        assert!(result.is_complete());
        assert_eq!(headers[0], httparse::EMPTY_HEADER);
    }

    #[test]
    fn header_per_line() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"Content-Length: 42\n";
        let result = httparse::parse_headers(input, &mut headers).unwrap();

        assert!(result.is_partial());
        assert_eq!(headers[0].name, "Content-Length");
        assert_eq!(headers[0].value, b"42");
    }

    #[test]
    fn header_per_line_complete() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"Content-Length: 42\r\n\r\n";
        let result = httparse::parse_headers(input, &mut headers).unwrap();

        assert!(result.is_complete());
        assert_eq!(result.unwrap().0, input.len());
        assert_eq!(headers[0].name, "Content-Length");
        assert_eq!(headers[0].value, b"42");
    }

    #[test]
    fn complete_with_body() {
        let mut headers = [httparse::EMPTY_HEADER];

        let input = b"Foo: Bar\r\n\r\nbody";
        let result = httparse::parse_headers(input, &mut headers).unwrap();
        let start_index = result.unwrap().0;

        assert!(result.is_complete());
        assert_eq!(start_index, input.strip_suffix(b"body").unwrap().len());
        assert_eq!(&input[start_index..], b"body");
        assert_eq!(headers[0].name, "Foo");
        assert_eq!(headers[0].value, b"Bar");
    }
}
