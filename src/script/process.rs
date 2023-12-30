use crate::context::RequestContext;
use crate::uri::decode_percent_encoded;
use crate::variable::ToMetaVariable;
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
    let server = context.server();
    let client = context.client();
    let request = context.request();

    let headers = request.headers.iter().map(|(key, value)| {
        (
            key.as_str().to_meta_variable(Some("HTTP")),
            String::from_utf8_lossy(value.as_bytes()).to_string(),
        )
    });

    let Some((script, extra_path)) = server.script_filename(request.uri.path()) else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::from("Requested a non-CGI script path."))
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData));
    };

    // <scheme> "://" <server-name> ":" <server-port>
    //                    <script-path> <extra-path> "?" <query-string>
    let script_name = format!("/cgi-bin/{}", script.display());
    let script_uri = format!(
        "{}://{}:{}{}{}?{}",
        server.scheme(),
        server.server_name(),
        server.port(),
        &script_name,
        &extra_path,
        request.uri.query().unwrap_or("")
    );

    let mut command = Command::new(&script);

    if request.method == http::Method::GET || request.method == http::Method::HEAD {
        if let Some(query) = request.uri.query() {
            if !query.contains('=') {
                for search_word in query.split('+') {
                    match decode_percent_encoded(search_word) {
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
        .current_dir(server.working_directory())
        .env_clear()
        .env("PATH", server.path())
        .env("SERVER_SOFTWARE", server.software())
        .env("GATEWAY_INTERFACE", "CGI/1.1")
        .env("SERVER_PROTOCOL", format!("{:?}", request.version))
        .env("SCRIPT_URI", script_uri)
        .env("SCRIPT_NAME", script_name)
        .env("SERVER_NAME", server.server_name())
        .env("SERVER_ADDR", server.ip_address())
        .env("SERVER_PORT", server.port())
        .env("REMOTE_ADDR", client.remote_ip_address())
        .env("REMOTE_PORT", client.remote_port())
        .env("REQUEST_METHOD", request.method.as_str())
        .envs(headers)
        .envs(request.uri.query().map(|q| ("QUERY_STRING", q)))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    if !extra_path.is_empty() {
        match decode_percent_encoded(extra_path) {
            Ok(path_info) => {
                command.env("PATH_INFO", path_info.as_str());
                command.env("PATH_TRANSLATED", server.translate_path(path_info.as_str()));
            }
            Err(path_info) => {
                command.env("PATH_INFO", path_info);
                command.env("PATH_TRANSLATED", server.translate_path(path_info));
            }
        }
    }

    if body.len() > 0 {
        command.env("CONTENT_LENGTH", body.len().to_string());

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
