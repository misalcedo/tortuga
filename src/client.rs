use crate::context::ServerContext;
use std::io;
use std::net::SocketAddr;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::{pin, select};

pub struct Client {
    context: Arc<ServerContext>,
    stream: TcpStream,
    remote_address: SocketAddr,
}

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
            let (reader, mut writer) = self.stream.split();
            let mut reader = BufReader::new(reader);

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
