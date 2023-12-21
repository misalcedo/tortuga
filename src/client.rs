use crate::context::ServerContext;
use std::io;
use std::net::SocketAddr;
use std::process::Stdio;
use std::sync::Arc;
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
const MAX_HEADER_BYTES: usize = 1024 * 32;

const CRLF: &'static str = "\r\n";
const HTTP_VERSION: &'static str = "HTTP/1.1";

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
                    return Ok(());
                }
            }

            reader = reader.into_inner().take(MAX_HEADER_BYTES as u64);
            let mut headers = String::with_capacity(1024 * 4);
            let mut line_start = 0;

            while &headers[line_start..] != CRLF {
                line_start = headers.len();

                if reader.read_line(&mut headers).await? == 0 {
                    return Ok(());
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
                .env("SERVER_PROTOCOL", "HTTP/1.1")
                .env("SCRIPT_FILENAME", self.context.script_filename())
                .env("SCRIPT_NAME", self.context.script_name())
                .env("SERVER_ADDR", self.context.ip_address())
                .env("SERVER_PORT", self.context.port())
                .env("REMOTE_ADDR", self.remote_address.ip().to_string())
                .env("REMOTE_PORT", self.remote_address.port().to_string())
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
