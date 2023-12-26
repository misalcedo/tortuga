use crate::service;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContext;

#[repr(transparent)]
#[derive(Clone)]
pub struct ShutdownSignal(CancellationToken);

impl ShutdownSignal {
    pub fn shutdown(self) {
        self.0.cancel()
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
                let (stream, remote_address) = select! {
                    _ = self.shutdown.0.cancelled() => {
                        break;
                    }
                    result = self.listener.accept() => {
                        result?
                    }
                };
                let handler = service::CommonGatewayInterface::new(context.clone(), remote_address);

                tokio::spawn(handler.run(stream));
            }

            Ok(())
        });

        self.runtime.shutdown_timeout(Duration::from_secs(5));

        result
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
    fn hello_world() {
        let server = Server::new(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let signal = server.shutdown();
        let address = server.listener.local_addr().unwrap();

        let thread = thread::spawn(|| server.serve("./examples/hello.cgi".into()));

        let mut client = TcpStream::connect_timeout(&address, Duration::from_secs(1)).unwrap();

        for _ in 0..10 {
            let mut output = vec![0; 1024];

            let response_start = "HTTP/1.1 200 OK\r\ncontent-length: 14\r\ndate: ";
            let response_end = " GMT\r\n\r\nHello, World!\n";

            client
                .write_all(
                    b"GET /%20foo?--abc%205 HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\nAccept: */*\r\n\r\n",
                )
                .unwrap();

            client.read(&mut output).unwrap();

            let response = String::from_utf8_lossy(output.as_slice());
            let end = response.find('\0').unwrap_or_else(|| response.len());

            assert_eq!(&response[..response_start.len()], response_start);
            assert_eq!(&response[(end - response_end.len())..end], response_end);
        }

        signal.shutdown();
        thread.join().unwrap().unwrap();
    }

    #[test]
    fn validate() {
        let server = Server::new(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let signal = server.shutdown();
        let address = server.listener.local_addr().unwrap();

        let thread = thread::spawn(|| server.serve("./examples/assert.cgi".into()));

        let mut client = TcpStream::connect_timeout(&address, Duration::from_secs(1)).unwrap();

        let mut output = vec![0; 1024];

        let response_start = "HTTP/1.1 200 OK\r\ncontent-length: 6\r\ndate: ";
        let response_end = " GMT\r\n\r\nfoobar";

        client
            .write_all(
                b"POST /%20foo?--abc%205 HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\nAccept: */*\r\ncontent-length: 6\r\ncontent-type: application/octet-stream\r\n\r\nfoobar",
            )
            .unwrap();

        client.read(&mut output).unwrap();

        let response = String::from_utf8_lossy(output.as_slice());
        let end = response.find('\0').unwrap_or_else(|| response.len());

        assert_eq!(&response[..response_start.len()], response_start);
        assert_eq!(&response[(end - response_end.len())..end], response_end);

        signal.shutdown();
        thread.join().unwrap().unwrap();
    }

    #[test]
    fn status() {
        let server = Server::new(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let signal = server.shutdown();
        let address = server.listener.local_addr().unwrap();

        let thread = thread::spawn(|| server.serve("./examples/status.cgi".into()));

        let mut client = TcpStream::connect_timeout(&address, Duration::from_secs(1)).unwrap();

        let mut output = vec![0; 1024];

        let response_start = "HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\ndate: ";
        let response_end = " GMT\r\n\r\n";

        client
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nStatus: 404\r\n\r\n\r\n")
            .unwrap();

        client.read(&mut output).unwrap();

        let response = String::from_utf8_lossy(output.as_slice());
        let end = response.find('\0').unwrap_or_else(|| response.len());

        assert_eq!(&response[..response_start.len()], response_start);
        assert_eq!(&response[(end - response_end.len())..end], response_end);

        signal.shutdown();
        thread.join().unwrap().unwrap();
    }
}
