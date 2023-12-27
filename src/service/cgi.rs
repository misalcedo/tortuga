use crate::context::ServerContext;
use crate::variable::ToMetaVariable;
use http_body_util::{BodyExt, Full};
use httparse::Status;
use hyper::body::{Body, Bytes, Incoming};
use hyper::http::{HeaderName, HeaderValue};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::future::Future;
use std::io;
use std::mem::size_of;
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
        let mut input = body
            .collect()
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?
            .to_bytes();

        let headers = request.headers.iter().map(|(key, value)| {
            (
                key.as_str().to_meta_variable(Some("HTTP")),
                String::from_utf8_lossy(value.as_bytes()).to_string(),
            )
        });

        let Some((script, extra_path)) = context.script_filename(request.uri.path()) else {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::from("Requested a non-CGI script path."))
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidData));
        };

        let mut command = Command::new(&script);

        if request.method == http::Method::GET || request.method == http::Method::HEAD {
            if let Some(query) = request.uri.query() {
                if !query.contains('=') {
                    for search_word in query.split('+') {
                        match decode_path(search_word) {
                            Ok(q) => {
                                command.args(q.split(' '));
                            }
                            Err(q) => {
                                command.args(q.split(' '));
                            }
                        }
                    }
                }
            }
        }

        command
            .kill_on_drop(true)
            .current_dir(context.working_directory())
            .env_clear()
            .env("PATH", context.path())
            .env("SERVER_SOFTWARE", context.software())
            .env("GATEWAY_INTERFACE", "CGI/1.1")
            .env("SERVER_PROTOCOL", format!("{:?}", request.version))
            .env("SCRIPT_NAME", format!("/cgi-bin/{}", script.display()))
            .env("SERVER_NAME", context.server_name())
            .env("SERVER_ADDR", context.ip_address())
            .env("SERVER_PORT", context.port())
            .env("REMOTE_ADDR", remote_address.ip().to_string())
            .env("REMOTE_PORT", remote_address.port().to_string())
            .env("REQUEST_METHOD", request.method.as_str())
            .envs(headers)
            .envs(request.uri.query().map(|q| ("QUERY_STRING", q)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());

        if !extra_path.is_empty() {
            match decode_path(extra_path) {
                Ok(path_info) => {
                    command.env("PATH_INFO", path_info.as_str());
                    command.env(
                        "PATH_TRANSLATED",
                        context.translate_path(path_info.as_str()),
                    );
                }
                Err(path_info) => {
                    command.env("PATH_INFO", path_info);
                    command.env("PATH_TRANSLATED", context.translate_path(path_info));
                }
            }
        }

        if input.len() > 0 {
            command.env("CONTENT_LENGTH", input.len().to_string());

            if let Some(Ok(value)) = request
                .headers
                .get(hyper::header::CONTENT_TYPE)
                .map(HeaderValue::to_str)
            {
                command.env("CONTENT_TYPE", value);
            }
        }

        let mut child = command.spawn()?;

        let mut stdin = child.stdin.take();
        let mut stdout = child.stdout.take();

        let cancel = CancellationToken::new();
        let stdin_cancel = cancel.child_token();
        let _cancel_guard = cancel.drop_guard();

        tokio::spawn(async move {
            if let Some(mut stdin) = stdin.take() {
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

                if output.is_empty() {
                    *response.status_mut() = StatusCode::BAD_GATEWAY;
                }

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
                                            if !name.as_str().starts_with("x-cgi-") {
                                                response.headers_mut().insert(name, value);
                                            }
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

fn decode_path(s: &str) -> Result<String, &str> {
    if !s.contains('%') {
        return Err(s);
    }

    let mut path = Vec::with_capacity(s.len());
    let mut buffer = [0u8; size_of::<char>()];
    let mut character = String::with_capacity(2);
    let mut characters = s.chars();

    while let Some(c) = characters.next() {
        match c {
            '+' => {
                path.extend_from_slice(' '.encode_utf8(&mut buffer).as_bytes());
            }
            '%' => match (characters.next(), characters.next()) {
                (Some(a), Some(b)) => {
                    character.clear();
                    character.push(a);
                    character.push(b);

                    match u8::from_str_radix(character.as_str(), 16) {
                        Ok(decoded) => path.push(decoded),
                        Err(_) => return Err(s),
                    }
                }
                _ => return Err(s),
            },
            _ => {
                path.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
            }
        }
    }

    String::from_utf8(path).map_err(|_| s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_characters() {
        assert_eq!(decode_path("abc"), Err("abc"));
        assert_eq!(decode_path("%2"), Err("%2"));
        assert_eq!(decode_path("%20%26").unwrap(), " &");
        assert_eq!(decode_path("%C6%92").unwrap(), "ƒ");
    }

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
