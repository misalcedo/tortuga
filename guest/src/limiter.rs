use std::io::{Read, Write};

#[derive(Clone, Debug, Default)]
pub struct IoLimiter<IO> {
    length: usize,
    io: IO,
}

impl<IO> IoLimiter<IO> {
    pub fn new(io: IO, length: usize) -> Self {
        IoLimiter { length, io }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn resize(&mut self, length: usize) {
        self.length = length;
    }

    pub fn get_mut(&mut self) -> &mut IO {
        &mut self.io
    }

    pub fn finish(self) -> IO {
        self.io
    }
}

impl<W: Write> Write for IoLimiter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_to_write = self.length.min(buf.len());
        let bytes_written = self.io.write(&buf[..bytes_to_write])?;

        self.length -= bytes_written;

        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }
}

impl<R> Read for IoLimiter<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_to_read = self.length.min(buf.len());
        let bytes_read = self.io.read(&mut buf[..bytes_to_read])?;

        self.length -= bytes_read;

        Ok(bytes_read)
    }
}
