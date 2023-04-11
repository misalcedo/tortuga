#[derive(Debug)]
pub struct InMemoryStream<Directionality> {
    identifier: u64,
    buffer: Cursor<Vec<u8>>,
    marker: PhantomData<Directionality>,
}

impl InMemoryStream<Bidirectional> {
    pub fn primary() -> Self {
        InMemoryStream {
            identifier: 0,
            buffer: Default::default(),
            marker: Default::default(),
        }
    }

    pub fn new(identifier: NonZeroU64) -> Self {
        InMemoryStream {
            identifier: identifier.get(),
            buffer: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<RW> Seek for InMemoryStream<RW> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.buffer.seek(pos)
    }
}

impl<RW> InMemoryStream<RW>
where
    RW: Readable + Writable,
{
    pub fn read_only(self) -> InMemoryStream<ReadOnly> {
        InMemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn write_only(self) -> InMemoryStream<WriteOnly> {
        InMemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }
}

impl<RW> InMemoryStream<RW> {
    pub fn read_write(mut self) -> InMemoryStream<Bidirectional> {
        InMemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn writable(mut self) -> InMemoryStream<WriteOnly> {
        let bytes = self.buffer.get_ref().len();

        self.buffer.set_position(bytes as u64);

        InMemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }

    pub fn readable(mut self) -> InMemoryStream<ReadOnly> {
        self.buffer.set_position(0);

        InMemoryStream {
            identifier: self.identifier,
            buffer: self.buffer,
            marker: Default::default(),
        }
    }
}

impl<R> Read for InMemoryStream<R>
where
    R: Readable,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl<W> Write for InMemoryStream<W>
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

    #[test]
    fn in_memory() {
        let bytes = b"Hello, World!";
        let mut stream = InMemoryStream::primary().write_only();

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
        let mut stream = InMemoryStream::primary();

        stream.write_all(bytes).unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), bytes);
    }
}
