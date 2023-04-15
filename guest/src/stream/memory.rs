use std::io::{Cursor, Read, Write};
use std::marker::PhantomData;

use crate::stream::{Readable, Writable};
use crate::{Bidirectional, ReadOnly, WriteOnly};

#[derive(Clone, Debug, PartialEq)]
pub struct Stream<Directionality> {
    reader: Cursor<Vec<u8>>,
    writer: Cursor<Vec<u8>>,
    marker: PhantomData<Directionality>,
}

impl Default for Stream<Bidirectional> {
    fn default() -> Self {
        Stream {
            reader: Default::default(),
            writer: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<B> From<B> for Stream<Bidirectional>
where
    B: AsRef<[u8]>,
{
    fn from(value: B) -> Self {
        Stream {
            reader: Cursor::new(Vec::from(value.as_ref())),
            writer: Default::default(),
            marker: Default::default(),
        }
    }
}

impl Stream<Bidirectional> {
    pub fn primary() -> Self {
        Stream::default()
    }

    pub fn new() -> Self {
        Stream::default()
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.reader, &mut self.writer);

        let length = self.writer.get_ref().len() as u64;

        self.reader.set_position(0);
        self.writer.set_position(length);
    }
}

impl<RW> Stream<RW>
where
    RW: Readable + Writable,
{
    pub fn split(self) -> (Stream<ReadOnly>, Stream<WriteOnly>) {
        let read = Stream {
            reader: self.reader,
            writer: Default::default(),
            marker: Default::default(),
        };
        let write = Stream {
            reader: Default::default(),
            writer: self.writer,
            marker: Default::default(),
        };

        (read, write)
    }
}

impl<R> Read for Stream<R>
where
    R: Readable,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<W> Write for Stream<W>
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
    use super::*;

    #[test]
    fn in_memory_read_write() {
        let bytes = b"Hello, World!";
        let mut stream = Stream::default();

        stream.write_all(bytes).unwrap();
        stream.swap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }

    #[test]
    fn swap() {
        let bytes = b"Hello, World!";
        let mut stream = Stream::default();

        stream.write_all(bytes).unwrap();
        stream.swap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }
}
