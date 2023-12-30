use crate::context::RequestContext;
use bytes::Bytes;
use http_body_util::Full;
use httparse::Status;
use hyper::http::{HeaderName, HeaderValue};
use hyper::{Response, StatusCode};
use std::io;
use std::process::Stdio;
use std::str::FromStr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;

pub async fn serve(context: RequestContext, body: Bytes) -> io::Result<Response<Full<Bytes>>> {
    let mut child = Command::new(context.script())
        .kill_on_drop(true)
        .current_dir(context.working_directory())
        .args(context.arguments())
        .env_clear()
        .envs(context.variables())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut stdin = child.stdin.take();
    let mut stdout = child.stdout.take();

    let cancel = CancellationToken::new();
    let stdin_cancel = cancel.child_token();
    let _cancel_guard = cancel.drop_guard();

    let mut input = body.clone();

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

            let mut response = Response::new(Full::from("Unable to wait for child process."));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(response)
        }
        Err(_) => {
            child.kill().await?;

            let mut response = Response::new(Full::from("Request timed out."));
            *response.status_mut() = StatusCode::GATEWAY_TIMEOUT;
            Ok(response)
        }
    }
}
