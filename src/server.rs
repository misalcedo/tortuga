use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::runtime::Runtime;
use tokio::{pin, select};

use crate::about;

pub struct Server {
    runtime: Runtime,
    listener: TcpListener,
}

impl Server {
    pub fn new(address: SocketAddr) -> io::Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()?;
        let listener = runtime.block_on(TcpListener::bind(address))?;

        Ok(Self { runtime, listener })
    }

    pub fn serve(self, script: PathBuf) -> io::Result<()> {
        // Context
        let address = self.listener.local_addr()?;
        let script_path = script.canonicalize()?;
        let ip_address = address.ip().to_string();
        let port = address.port().to_string();
        let path: &'static str = env!("PATH");
        let software = format!("{}/{}", about::PROGRAM, about::VERSION);
        let signature = format!(
            "<address>{} Server at {} Port {}</address>\n",
            software, ip_address, port
        );

        self.runtime.block_on(async {
            loop {
                let (mut client, remote_address) = self.listener.accept().await?;
                let mut command = Command::new(&script_path);
                command
                    .env_clear()
                    .env("PATH", path)
                    .env("SERVER_SOFTWARE", software.as_str())
                    .env("SERVER_SIGNATURE", signature.as_str())
                    .env("GATEWAY_INTERFACE", "CGI/1.1")
                    .env("SERVER_PROTOCOL", "HTTP/1.1")
                    .env("SCRIPT_FILENAME", script_path.as_os_str())
                    .env("SCRIPT_NAME", script.as_os_str())
                    .env("SERVER_ADDR", ip_address.as_str())
                    .env("SERVER_PORT", port.as_str())
                    .env("REMOTE_ADDR", remote_address.ip().to_string())
                    .env("REMOTE_PORT", remote_address.port().to_string());

                tokio::spawn(async move {
                    {
                        let (mut reader, mut writer) = client.split();

                        let mut child = command
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
                            }
                        }
                    }

                    client.shutdown().await
                });
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn new_connections() {
        let server = Server::new(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let address = server.listener.local_addr().unwrap();

        let thread = thread::spawn(|| server.serve("./examples/hello.cgi".into()));

        let mut client = TcpStream::connect_timeout(&address, Duration::from_secs(1)).unwrap();
        let mut output = String::new();

        client.write_all(b"Hi!").unwrap();
        client.read_to_string(&mut output).unwrap();

        assert_eq!(output.as_str(), "\nHello, World!\n");
    }
}
