use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::num::NonZeroU64;

use crate::stream::{Readable, Writable};
use crate::{Bidirectional, ReadOnly, WriteOnly};

#[derive(Debug, PartialEq, Clone)]
pub struct MemoryStream<Directionality> {
    identifier: u64,
    buffer: Cursor<Vec<u8>>,
    marker: PhantomData<Directionality>,
}

impl Default for MemoryStream<Bidirectional> {
    fn default() -> Self {
        Self::primary()
    }
}

impl MemoryStream<Bidirectional> {
    pub fn primary() -> Self {
        MemoryStream {
            identifier: 0,
            buffer: Default::default(),
            marker: Default::default(),
        }
    }

    pub fn new(identifier: NonZeroU64) -> Self {
        MemoryStream {
            identifier: identifier.get(),
            buffer: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<RW> Seek for MemoryStream<RW> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.buffer.seek(pos)
    }
}

impl<RW> MemoryStream<RW>
where
    RW: Readable + Writable,
{
    pub fn read_only(self) -> MemoryStream<ReadOnly> {
        MemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn write_only(self) -> MemoryStream<WriteOnly> {
        MemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }
}

impl<RW> MemoryStream<RW> {
    pub fn read_write(self) -> MemoryStream<Bidirectional> {
        MemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn writable(mut self) -> MemoryStream<WriteOnly> {
        let bytes = self.buffer.get_ref().len();

        self.buffer.set_position(bytes as u64);

        MemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn readable(mut self) -> MemoryStream<ReadOnly> {
        self.buffer.set_position(0);

        MemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn position(&self) -> usize {
        self.buffer.position() as usize
    }

    pub fn len(&self) -> usize {
        self.buffer.get_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.get_ref().is_empty()
    }
}

impl<R> Read for MemoryStream<R>
where
    R: Readable,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl<W> Write for MemoryStream<W>
where
    W: Writable,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU64;

    #[test]
    fn in_memory() {
        let bytes = b"Hello, World!";
        let mut stream = MemoryStream::primary().write_only();

        stream.write_all(bytes).unwrap();

        let mut stream = stream.readable();
        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);

        let mut stream = stream.writable();

        stream.write_all(bytes).unwrap();

        let mut stream = stream.readable();
        let mut extended_buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut extended_buffer).unwrap();
        buffer.write_all(bytes).unwrap();

        assert_eq!(
            buffer.get_ref().as_slice(),
            extended_buffer.get_ref().as_slice()
        );
    }

    #[test]
    fn in_memory_read_write() {
        let bytes = b"Hello, World!";
        let mut stream = MemoryStream::new(NonZeroU64::new(1).unwrap());

        stream.write_all(bytes).unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }
}
