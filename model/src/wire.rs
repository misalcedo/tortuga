use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Wire: io::Write + io::Read + Send + Sync {
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;
}
