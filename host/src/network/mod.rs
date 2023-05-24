use async_trait::async_trait;
use std::net::TcpStream;

pub use pipe::Pipe;
pub use stream::DuplexStream;

mod pipe;
mod ring;
mod stream;

pub trait DuplexBuffer {
    fn remaining(&self) -> usize;

    fn remaining_mut(&self) -> usize;
}

#[async_trait]
pub trait Network: Clone + Send + Sync {
    async fn add(&mut self, origin: &str, guest: ()) -> Option<()>;

    async fn connect(&mut self, origin: &str) -> Result<DuplexStream, TcpStream>;
}
