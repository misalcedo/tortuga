use crate::asynchronous;
use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Wire: Send + Sync {
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;

    async fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()>;

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;

    async fn write_all(&mut self, buffer: &[u8]) -> io::Result<()>;

    async fn flush(&mut self) -> io::Result<()>;
}

#[async_trait]
impl<W> Wire for W
where
    W: asynchronous::Read + asynchronous::Write + Send + Sync,
{
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        asynchronous::Read::read(self, buffer).await
    }

    async fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        asynchronous::Read::read_exact(self, buffer).await
    }

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        asynchronous::Write::write(self, buffer).await
    }

    async fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
        asynchronous::Write::write_all(self, buffer).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        asynchronous::Write::flush(self).await
    }
}
