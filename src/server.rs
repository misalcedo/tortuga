use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::{Arc, Once};
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::runtime::Runtime;

#[repr(transparent)]
pub struct ShutdownSignal(Arc<Once>);

impl ShutdownSignal {
    pub fn shutdown(self) {
        self.0.call_once(|| {})
    }
}

impl Drop for ShutdownSignal {
    fn drop(&mut self) {
        self.0.call_once(|| {})
    }
}

pub struct Server {
    runtime: Runtime,
    listener: TcpListener,
    signal: Arc<Once>,
}

impl Server {
    pub fn new(address: SocketAddr) -> io::Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()?;
        let listener = runtime.block_on(TcpListener::bind(address))?;

        Ok(Self {
            runtime,
            listener,
            signal: Arc::new(Once::new()),
        })
    }

    pub fn shutdown_signal(&self) -> ShutdownSignal {
        ShutdownSignal(self.signal.clone())
    }

    pub fn serve(self, script: PathBuf) -> io::Result<()> {
        let script_path = script.canonicalize()?;

        self.runtime.block_on(async {
            loop {
                if self.signal.is_completed() {
                    break;
                }

                let (stream, _) = self.listener.accept().await?;
                let mut command = Command::new(&script_path);

                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.into_split();

                    let mut child = command
                        .env_clear()
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::inherit())
                        .spawn()?;

                    let mut reader_task = None;

                    // TODO: reader does not finish
                    if let Some(mut stdin) = child.stdin.take() {
                        reader_task = Some(tokio::spawn(async move {
                            tokio::io::copy(&mut reader, &mut stdin).await
                        }));
                    }

                    if let Some(mut stdout) = child.stdout.take() {
                        tokio::spawn(
                            async move { tokio::io::copy(&mut stdout, &mut writer).await },
                        );
                    }

                    child.wait().await?;
                    reader_task?.abort();
                });
            }

            Ok(())
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
        let signal = server.shutdown_signal();

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
