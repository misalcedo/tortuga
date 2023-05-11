use async_trait::async_trait;
use std::io;

#[async_trait]
pub trait Wire: Send + Sync {
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;

    async fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()>;

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;

    async fn write_all(&mut self, buffer: &[u8]) -> io::Result<()>;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Counter(usize, usize);

impl Counter {
    pub fn bytes_read(&self) -> usize {
        self.0
    }

    pub fn bytes_written(&self) -> usize {
        self.1
    }
}

#[async_trait]
impl Wire for Counter {
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.0 += buffer.len();

        Ok(buffer.len())
    }

    async fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        self.0 += buffer.len();

        Ok(())
    }

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.1 += buffer.len();

        Ok(buffer.len())
    }

    async fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
        self.1 += buffer.len();

        Ok(())
    }
}
