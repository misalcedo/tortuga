use crate::context::ServerContext;
use regex::Regex;
use std::io;
use std::net::SocketAddr;
use std::process::Stdio;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::{pin, select};

pub struct Client {
    context: Arc<ServerContext>,
    stream: TcpStream,
    remote_address: SocketAddr,
}

const MAX_REQUEST_BYTES: usize = 1024 * 16;
const MAX_HEADERS_BYTES: usize = 1024 * 32;
const MAX_HEADER_BYTES: usize = 1024 * 16;

const CRLF: &'static str = "\r\n";
const HTTP_VERSION: &'static str = "HTTP/1.1";

static REQUEST_LINE: OnceLock<Regex> = OnceLock::new();

impl Client {
    pub fn new(context: Arc<ServerContext>, stream: TcpStream, remote_address: SocketAddr) -> Self {
        Self {
            context,
            stream,
            remote_address,
        }
    }

    pub async fn run(mut self) -> io::Result<()> {
        {
            let (reader, writer) = self.stream.split();
            let mut reader = BufReader::new(reader).take(MAX_REQUEST_BYTES as u64);
            let mut writer = BufWriter::new(writer);

            let mut request = String::with_capacity(512);
            while request.is_empty() || request.as_str() == CRLF {
                request.clear();

                if reader.read_line(&mut request).await? == 0 {
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
                }
            }

            let request_regex = REQUEST_LINE
                .get_or_init(|| {
                    Regex::new(
                    r"^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (.+) HTTP/1.1\r\n$",
                )
                .expect("Invalid regular expression for request line.")
                })
                .clone();
            let request_captures = request_regex
                .captures(request.as_str())
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

            let method = &request_captures[1];
            let uri = &request_captures[2];
            let query = uri.split_once('?').map(|(_, q)| q);

            reader.set_limit(MAX_HEADERS_BYTES as u64);

            let mut header = String::with_capacity(1024 * 4);

            while header != CRLF {
                header.clear();

                let header_bytes = reader.read_line(&mut header).await?;

                if header_bytes == 0 {
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
                } else if header_bytes >= MAX_HEADER_BYTES {
                    return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
                }
            }

            writer.write_all(HTTP_VERSION.as_bytes()).await?;
            writer.write_all(b" 200 OK").await?;
            writer.write_all(CRLF.as_bytes()).await?;
            writer.flush().await?;

            let mut child = Command::new(self.context.script_filename())
                .kill_on_drop(true)
                .env_clear()
                .env("PATH", self.context.path())
                .env("SERVER_SOFTWARE", self.context.software())
                .env("GATEWAY_INTERFACE", "CGI/1.1")
                .env("SERVER_PROTOCOL", HTTP_VERSION)
                .env("SCRIPT_FILENAME", self.context.script_filename())
                .env("SCRIPT_NAME", self.context.script_name())
                .env("SERVER_ADDR", self.context.ip_address())
                .env("SERVER_PORT", self.context.port())
                .env("REMOTE_ADDR", self.remote_address.ip().to_string())
                .env("REMOTE_PORT", self.remote_address.port().to_string())
                .env("PATH_INFO", uri)
                .env("REQUEST_METHOD", method)
                .envs(query.map(|q| ("QUERY_STRING", q)))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?;

            let mut stdin = child.stdin.take().map(BufWriter::new);
            let mut stdout = child.stdout.take().map(BufReader::new);

            let reader_task = async {
                if let Some(mut stdin) = stdin.as_mut() {
                    tokio::io::copy(&mut reader, &mut stdin).await
                } else {
                    Ok(0)
                }
            };

            let writer_task = async {
                if let Some(mut stdout) = stdout.as_mut() {
                    tokio::io::copy(&mut stdout, &mut writer).await
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
                        break;
                    }
                    _ = &mut reader_task, if !reader_done => {
                        reader_done = true;
                    }
                    _ = &mut writer_task, if !writer_done => {
                        writer_done = true;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {
                        child.start_kill()?;
                    }
                }
            }
        }

        self.stream.shutdown().await
    }
}
