use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::num::NonZeroU64;

use crate::stream::{Readable, Writable};
use crate::{Bidirectional, ReadOnly, WriteOnly};

#[derive(Debug, PartialEq, Clone)]
pub struct MemoryStream<Directionality> {
    identifier: u64,
    reader: Cursor<Vec<u8>>,
    writer: Cursor<Vec<u8>>,
    marker: PhantomData<Directionality>,
}

impl Default for MemoryStream<Bidirectional> {
    fn default() -> Self {
        Self::primary()
    }
}

impl From<u64> for MemoryStream<Bidirectional> {
    fn from(identifier: u64) -> Self {
        MemoryStream {
            identifier,
            reader: Default::default(),
            writer: Default::default(),
            marker: Default::default(),
        }
    }
}

impl From<Option<NonZeroU64>> for MemoryStream<Bidirectional> {
    fn from(identifier: Option<NonZeroU64>) -> Self {
        MemoryStream {
            identifier: identifier.map(NonZeroU64::get).unwrap_or_default(),
            reader: Default::default(),
            writer: Default::default(),
            marker: Default::default(),
        }
    }
}

impl MemoryStream<Bidirectional> {
    pub fn primary() -> Self {
        Self::from(0u64)
    }

    pub fn new(identifier: NonZeroU64) -> Self {
        Self::from(identifier.get())
    }

    pub fn with_reader(reader: impl AsRef<[u8]>) -> Self {
        MemoryStream {
            identifier: 0,
            reader: Cursor::new(Vec::from(reader.as_ref())),
            writer: Default::default(),
            marker: Default::default(),
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.reader, &mut self.writer);

        let length = self.writer.get_ref().len() as u64;

        self.reader.set_position(0);
        self.writer.set_position(length);
    }
}

impl Seek for MemoryStream<Bidirectional> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        // Auto-swap when the stream is done being written to.
        // This avoids the common case of having to swap after writing to the in-memory buffer.
        if self.reader.get_ref().is_empty() && !self.writer.get_ref().is_empty() {
            self.swap();
        }

        self.reader.seek(pos)
    }
}

impl Seek for MemoryStream<ReadOnly> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }
}

impl<RW> MemoryStream<RW>
where
    RW: Readable + Writable,
{
    pub fn split(self) -> (MemoryStream<ReadOnly>, MemoryStream<WriteOnly>) {
        let read = MemoryStream {
            identifier: self.identifier,
            reader: self.reader,
            writer: Default::default(),
            marker: Default::default(),
        };
        let write = MemoryStream {
            identifier: self.identifier,
            reader: Default::default(),
            writer: self.writer,
            marker: Default::default(),
        };

        (read, write)
    }
}

impl<R> MemoryStream<R>
where
    R: Readable,
{
    pub fn writable(mut self) -> MemoryStream<WriteOnly> {
        let length = self.reader.get_ref().len() as u64;

        self.reader.set_position(length);

        MemoryStream {
            identifier: self.identifier,
            reader: self.writer,
            writer: self.reader,
            marker: Default::default(),
        }
    }
}

impl<W> MemoryStream<W>
where
    W: Writable,
{
    pub fn readable(mut self) -> MemoryStream<ReadOnly> {
        self.writer.set_position(0);

        MemoryStream {
            identifier: self.identifier,
            reader: self.writer,
            writer: self.reader,
            marker: Default::default(),
        }
    }
}

impl<RW> MemoryStream<RW> {
    pub fn position(&self) -> usize {
        self.reader.position() as usize
    }
}

impl<R> MemoryStream<R>
where
    R: Readable,
{
    pub fn remaining(&self) -> usize {
        self.reader.get_ref().len() - self.reader.position() as usize
    }

    pub fn len(&self) -> usize {
        self.reader.get_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.reader.get_ref().is_empty()
    }
}

impl<R> Read for MemoryStream<R>
where
    R: Readable,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<W> Write for MemoryStream<W>
where
    W: Writable,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::*;

    #[test]
    fn in_memory() {
        let bytes = b"Hello, World!";
        let mut stream = MemoryStream::primary();

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
        let mut stream = MemoryStream::from(NonZeroU64::new(1));

        stream.write_all(bytes).unwrap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream.readable(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }

    #[test]
    fn swap() {
        let bytes = b"Hello, World!";
        let mut stream = MemoryStream::from(NonZeroU64::new(1));

        stream.write_all(bytes).unwrap();
        stream.swap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }

    #[test]
    fn auto_swap() {
        let bytes = b"Hello, World!";
        let mut stream = MemoryStream::from(NonZeroU64::new(1));

        stream.write_all(bytes).unwrap();
        stream.rewind().unwrap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }
}
