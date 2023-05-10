use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Wire: Send + Sync {
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;

    async fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()>;

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;

    async fn write_all(&mut self, buffer: &[u8]) -> io::Result<()>;
}
