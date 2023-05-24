use std::collections::HashMap;
use std::future::Future;
use std::net::TcpStream;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll, Waker};

pub use pipe::Pipe;
pub use stream::DuplexStream;

mod pipe;
mod ring;
mod stream;

#[derive(Clone, Debug, Default)]
pub struct Listener {
    streams: Arc<RwLock<Vec<DuplexStream>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Future for Listener {
    type Output = DuplexStream;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut streams = RwLock::write(&self.streams).expect("listener streams lock was poisoned");

        match streams.pop() {
            None => {
                let mut waker = Mutex::lock(&self.waker).expect("listener waker lock was poisoned");
                *waker = Some(cx.waker().clone());
                Poll::Pending
            }
            Some(stream) => Poll::Ready(stream),
        }
    }
}

impl Listener {
    pub fn push(&mut self, stream: DuplexStream) {
        let mut streams = RwLock::write(&self.streams).expect("listener streams lock was poisoned");

        streams.push(stream);

        let mut waker = Mutex::lock(&self.waker).expect("listener waker lock was poisoned");

        if let Some(waker) = waker.take() {
            waker.wake();
        }
    }

    pub async fn pop(&mut self) -> DuplexStream {
        self.await
    }
}

#[derive(Clone, Debug)]
pub struct Network {
    capacity: usize,
    listeners: Arc<RwLock<HashMap<String, Listener>>>,
}

impl Network {
    pub fn new(capacity: usize) -> Self {
        Network {
            capacity,
            listeners: Arc::new(Default::default()),
        }
    }

    pub fn listen(&mut self, origin: &str) -> Listener {
        let mut listeners =
            RwLock::write(&self.listeners).expect("network listeners lock was poisoned");
        let listener = Listener::default();

        listeners.insert(Self::normalize_origin(origin), listener.clone());
        listener
    }

    pub async fn connect(&mut self, origin: &str) -> Result<DuplexStream, TcpStream> {
        let mut listeners =
            RwLock::write(&self.listeners).expect("network listeners lock was poisoned");

        match listeners.get_mut(Self::normalize_origin(origin).as_str()) {
            None => todo!("Implement connecting to remote origins."),
            Some(listener) => {
                let (client, server) = DuplexStream::new(self.capacity);

                listener.push(server);

                Ok(client)
            }
        }
    }

    fn normalize_origin(origin: &str) -> String {
        origin.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tortuga_model::asynchronous::{Read, Write};

    #[tokio::test]
    async fn basic() {
        let origin = "https://www.example.com";
        let from_client = b"Hello, world!";
        let from_server = b"Goodbye, world!";

        let mut network = Network::new(1024);
        let mut listener = network.listen(origin);

        let mut client = network.connect(origin).await.unwrap();
        let mut server = listener.pop().await;

        let mut buffer = vec![0u8; from_client.len()];

        client.write_all(from_client).await.unwrap();
        server.read_exact(buffer.as_mut_slice()).await.unwrap();

        assert_eq!(from_client, buffer.as_mut_slice());

        let mut buffer = vec![0u8; from_server.len()];

        server.write_all(from_server).await.unwrap();
        client.read_exact(buffer.as_mut_slice()).await.unwrap();

        assert_eq!(from_server, buffer.as_mut_slice());
    }

    #[tokio::test]
    async fn empty() {
        let mut network = Network::new(0);

        assert!(network.connect("https://www.example.com").await.is_err())
    }
}
