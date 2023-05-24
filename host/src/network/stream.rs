use crate::network::Pipe;
use async_trait::async_trait;
use std::io::{Read, Write};
use tortuga_model::asynchronous;

#[derive(Debug)]
pub struct DuplexStream {
    read: Pipe,
    write: Pipe,
}

impl DuplexStream {
    pub fn new(capacity: usize) -> (DuplexStream, DuplexStream) {
        let read = Pipe::new(capacity);
        let write = Pipe::new(capacity);

        let b = DuplexStream {
            read: write.clone(),
            write: read.clone(),
        };
        let a = DuplexStream { read, write };

        (a, b)
    }
}

impl Read for DuplexStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read.read(buf)
    }
}

impl Write for DuplexStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush()
    }
}

#[async_trait]
impl asynchronous::Read for DuplexStream {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        asynchronous::Read::read(&mut self.read, buf).await
    }
}

#[async_trait]
impl asynchronous::Write for DuplexStream {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        asynchronous::Write::write(&mut self.write, buf).await
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        asynchronous::Write::flush(&mut self.write).await
    }
}
