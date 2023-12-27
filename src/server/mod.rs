use crate::{service, ServeOptions};
use hyper::{Request, Response};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use http::{Method, StatusCode};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::select;

mod router;
mod shutdown;

use crate::context::ServerContext;
pub use shutdown::ShutdownSignal;
use router::Router;

///    The server acts as an application gateway.  It receives the request
///    from the client, selects a CGI script to handle the request, converts
///    the client request to a CGI request, executes the script and converts
///    the CGI response into a response for the client.  When processing the
///    client request, it is responsible for implementing any protocol or
///    transport level authentication and security.  The server MAY also
///    function in a 'non-transparent' manner, modifying the request or
///    response in order to provide some additional service, such as media
///    type transformation or protocol reduction.
///
///    The server MUST perform translations and protocol conversions on the
///    client request data required by this specification.  Furthermore, the
///    server retains its responsibility to the client to conform to the
///    relevant network protocol even if the CGI script fails to conform to
///    this specification.
///
///    If the server is applying authentication to the request, then it MUST
///    NOT execute the script unless the request passes all defined access
///    controls.
pub struct Server {
    context: Arc<ServerContext>,
    shutdown: ShutdownSignal,
    listener: TcpListener,
}

impl Server {
    pub async fn bind(mut options: ServeOptions) -> io::Result<Self> {
        let mut addresses =
            tokio::net::lookup_host(format!("{}:{}", options.hostname, options.port)).await?;
        let address = addresses.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                "unable to resolve interface to a local address",
            )
        })?;

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?;

        options.document_root = options.document_root.canonicalize()?;

        if options.cgi_bin.is_relative() {
            options.cgi_bin = options
                .document_root
                .join(&options.cgi_bin)
                .canonicalize()?;
        }

        Ok(Self {
            context: Arc::new(ServerContext::new(address, options)),
            shutdown: ShutdownSignal::new(),
            listener,
        })
    }

    pub fn shutdown_signal(&self) -> ShutdownSignal {
        self.shutdown.clone()
    }

    pub fn address(&self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn serve(self) -> io::Result<()> {
        loop {
            let (stream, remote_address) = select! {
                _ = self.shutdown.shutdown_requested() => {
                    break;
                }
                result = self.listener.accept() => {
                    result?
                }
            };

            let handler = http1::Builder::new()
                .serve_connection(TokioIo::new(stream), Router::new(self.context.clone(), remote_address));

            tokio::spawn(handler);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Component::CurDir;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn hello_world() {
        let mut client = connect_to_server().await;

        for _ in 0..10 {
            let mut output = vec![0; 1024];

            let response_start = "HTTP/1.1 200 OK\r\ncontent-length: 14\r\ndate: ";
            let response_end = " GMT\r\n\r\nHello, World!\n";

            client
                .write_all(
                    b"GET /cgi-bin/hello.cgi/%20foo?--abc%205 HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\nAccept: */*\r\n\r\n",
                ).await
                .unwrap();

            client.read(&mut output).await.unwrap();

            let response = String::from_utf8_lossy(output.as_slice());
            let end = response.find('\0').unwrap_or_else(|| response.len());

            assert_eq!(&response[..response_start.len()], response_start);
            assert_eq!(&response[(end - response_end.len())..end], response_end);
        }
    }

    #[tokio::test]
    async fn head() {
        let mut client = connect_to_server().await;

        let mut output = vec![0; 1024];

        let response_start = "HTTP/1.1 200 OK\r\ncontent-length: 0\r\ndate: ";
        let response_end = " GMT\r\n\r\n";

        client
            .write_all(
                b"HEAD /cgi-bin/hello.cgi/%20foo?--abc%205 HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\nAccept: */*\r\n\r\n",
            ).await
            .unwrap();

        client.read(&mut output).await.unwrap();

        let response = String::from_utf8_lossy(output.as_slice());
        let end = response.find('\0').unwrap_or_else(|| response.len());

        assert_eq!(&response[..response_start.len()], response_start);
        assert_eq!(&response[(end - response_end.len())..end], response_end);
    }

    #[tokio::test]
    async fn validate() {
        let mut client = connect_to_server().await;
        let mut output = vec![0; 1024];

        let response_start = "HTTP/1.1 200 OK\r\ncontent-length: 6\r\ndate: ";
        let response_end = " GMT\r\n\r\nfoobar";

        client
            .write_all(
                b"GET /cgi-bin/assert.cgi/%20foo?--abc%205+--d+42 HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\nAccept: */*\r\ncontent-length: 6\r\ncontent-type: application/octet-stream\r\n\r\nfoobar",
            ).await
            .unwrap();

        client.read(&mut output).await.unwrap();

        let response = String::from_utf8_lossy(output.as_slice());
        let end = response.find('\0').unwrap_or_else(|| response.len());

        assert_eq!(&response[..response_start.len()], response_start);
        assert_eq!(&response[(end - response_end.len())..end], response_end);
    }

    #[tokio::test]
    async fn status() {
        let mut client = connect_to_server().await;
        let mut output = vec![0; 1024];

        let response_start =
            "HTTP/1.1 404 Not Found\r\ncontent-type: text/html\r\ncontent-length: 0\r\ndate: ";
        let response_end = " GMT\r\n\r\n";

        client
            .write_all(
                b"GET /cgi-bin/status.cgi HTTP/1.1\r\nHost: localhost\r\nStatus: 404\r\n\r\n\r\n",
            )
            .await
            .unwrap();

        client.read(&mut output).await.unwrap();

        let response = String::from_utf8_lossy(output.as_slice());
        let end = response.find('\0').unwrap_or_else(|| response.len());

        assert_eq!(&response[..response_start.len()], response_start);
        assert_eq!(&response[(end - response_end.len())..end], response_end);
    }

    #[tokio::test]
    async fn not_cgi() {
        let mut client = connect_to_server().await;
        let mut output = vec![0; 1024];

        let response_start = "HTTP/1.1 404 Not Found\r\ncontent-length: 32\r\ndate: ";
        let response_end = " GMT\r\n\r\nRequested a non-CGI script path.";

        client
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n\r\n")
            .await
            .unwrap();

        client.read(&mut output).await.unwrap();

        let response = String::from_utf8_lossy(output.as_slice());
        let end = response.find('\0').unwrap_or_else(|| response.len());

        assert_eq!(&response[..response_start.len()], response_start);
        assert_eq!(&response[(end - response_end.len())..end], response_end);
    }

    async fn connect_to_server() -> TcpStream {
        let server = Server::bind(ServeOptions {
            document_root: "./examples".into(),
            cgi_bin: CurDir.as_os_str().into(),
            hostname: "localhost".to_string(),
            port: 0,
        })
        .await
        .unwrap();
        let address = server.address().unwrap();

        tokio::spawn(server.serve());

        TcpStream::connect(&address).await.unwrap()
    }
}
