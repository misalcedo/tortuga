use crate::IoLimiter;
use std::io::{Cursor, Read, Write};
use std::mem::size_of;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum FrameType {
    Data = 0x00,
    Header = 0x01,
}

impl TryFrom<u8> for FrameType {
    type Error = u8;

    fn try_from(kind: u8) -> Result<Self, Self::Error> {
        match kind {
            0x00 => Ok(FrameType::Data),
            0x01 => Ok(FrameType::Header),
            _ => Err(kind),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    kind: FrameType,
    length: usize,
}

impl Frame {
    pub fn new(kind: FrameType, length: usize) -> Self {
        Frame { kind, length }
    }

    pub fn bytes() -> usize {
        size_of::<u8>() + size_of::<u64>()
    }

    pub fn kind(&self) -> FrameType {
        self.kind
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[derive(Debug)]
pub struct Header<Stream> {
    buffer: Cursor<Vec<u8>>,
    stream: IoLimiter<Stream>,
}

impl<Stream> Header<Stream>
where
    Stream: Read,
{
    pub fn new(stream: Stream, length: usize) -> Self {
        Header {
            buffer: Default::default(),
            stream: IoLimiter::new(stream, length),
        }
    }
}

impl<Stream> Header<Stream> {
    pub fn buffer(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.buffer
    }

    pub fn inner(&mut self) -> &mut Stream {
        self.stream.get_mut()
    }

    pub fn finish(self) -> (Cursor<Vec<u8>>, Stream) {
        (self.buffer, self.stream.finish())
    }
}

impl<Stream> Read for Header<Stream>
where
    Stream: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.stream.read(buf)?;

        self.buffer.write(&buf[..bytes])?;

        Ok(bytes)
    }
}

impl<Stream> Write for Header<Stream>
where
    Stream: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
