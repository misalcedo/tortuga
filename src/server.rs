use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio::runtime::Runtime;
use tokio::{pin, select};
use tokio_util::sync::CancellationToken;

use crate::about;

#[repr(transparent)]
#[derive(Clone)]
pub struct ShutdownSignal(CancellationToken);

impl ShutdownSignal {
    pub fn shutdown(self) {
        self.0.cancel()
    }
}

struct ServerContext {
    script: PathBuf,
    script_path: PathBuf,
    ip_address: String,
    port: String,
    path: &'static str,
    software: String,
    signature: String,
}

impl ServerContext {
    fn new(address: SocketAddr, script: PathBuf) -> io::Result<Self> {
        let script_path = script.canonicalize()?;

        let ip_address = address.ip().to_string();
        let port = address.port().to_string();

        let path: &'static str = env!("PATH");

        let software = format!("{}/{}", about::PROGRAM, about::VERSION);
        let signature = format!(
            "<address>{} Server at {} Port {}</address>\n",
            software, ip_address, port
        );

        Ok(Self {
            script,
            script_path,
            ip_address,
            port,
            path,
            software,
            signature,
        })
    }
}

pub struct Server {
    runtime: Runtime,
    listener: TcpListener,
    shutdown: ShutdownSignal,
}

impl Server {
    pub fn new(address: SocketAddr) -> io::Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()?;
        let listener = runtime.block_on(TcpListener::bind(address))?;
        let shutdown = ShutdownSignal(CancellationToken::new());

        Ok(Self {
            runtime,
            listener,
            shutdown,
        })
    }

    pub fn shutdown(&self) -> ShutdownSignal {
        self.shutdown.clone()
    }

    pub fn serve(self, script: PathBuf) -> io::Result<()> {
        let address = self.listener.local_addr()?;
        let context = Arc::new(ServerContext::new(address, script)?);

        let result = self.runtime.block_on(async {
            loop {
                let (client, remote_address) = select! {
                    _ = self.shutdown.0.cancelled() => {
                        break;
                    }
                    result = self.listener.accept() => {
                        result?
                    }
                };

                tokio::spawn(Self::handle_client(context.clone(), remote_address, client));
            }

            Ok(())
        });

        self.runtime.shutdown_timeout(Duration::from_secs(5));

        result
    }

    async fn handle_client(
        context: Arc<ServerContext>,
        remote_address: SocketAddr,
        mut client: TcpStream,
    ) -> io::Result<()> {
        {
            let (mut reader, mut writer) = client.split();
            let mut child = Command::new(&context.script_path)
                .kill_on_drop(true)
                .env_clear()
                .env("PATH", &context.path)
                .env("SERVER_SOFTWARE", &context.software.as_str())
                .env("SERVER_SIGNATURE", &context.signature.as_str())
                .env("GATEWAY_INTERFACE", "CGI/1.1")
                .env("SERVER_PROTOCOL", "HTTP/1.1")
                .env("SCRIPT_FILENAME", &context.script_path.as_os_str())
                .env("SCRIPT_NAME", &context.script.as_os_str())
                .env("SERVER_ADDR", &context.ip_address.as_str())
                .env("SERVER_PORT", &context.port.as_str())
                .env("REMOTE_ADDR", remote_address.ip().to_string())
                .env("REMOTE_PORT", remote_address.port().to_string())
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

        client.shutdown().await
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
        let signal = server.shutdown();
        let address = server.listener.local_addr().unwrap();

        let thread = thread::spawn(|| server.serve("./examples/hello.cgi".into()));

        let mut client = TcpStream::connect_timeout(&address, Duration::from_secs(1)).unwrap();
        let mut output = String::new();

        client.write_all(b"Hi!").unwrap();
        client.read_to_string(&mut output).unwrap();

        signal.shutdown();
        thread.join().unwrap().unwrap();

        assert_eq!(output.as_str(), "\nHello, World!\n");
    }
}
