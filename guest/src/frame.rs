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
    stream: Stream,
}

impl<Stream> Header<Stream> {
    pub fn is_empty(&self) -> bool {
        self.buffer.position() >= self.buffer.get_ref().len() as u64
    }

    pub fn stream_mut(&mut self) -> &mut Stream {
        &mut self.stream
    }

    pub fn reset(&mut self) {
        self.buffer.set_position(0);
    }

    pub fn finish(self) -> Stream {
        self.stream
    }
}

impl<Stream> Header<Stream>
where
    Stream: Read,
{
    pub fn new(head: Vec<u8>, stream: Stream) -> Self {
        Header {
            buffer: Cursor::new(head),
            stream,
        }
    }
}

impl<Stream> Read for Header<Stream>
where
    Stream: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.position() < self.buffer.get_ref().len() as u64 {
            self.buffer.read(buf)
        } else {
            self.stream.read(buf)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Destination, Method, Request, Source};

    #[test]
    fn read_message() {
        let mut stream = Cursor::new(Vec::new());
        let mut buffer = Cursor::new(Vec::new());

        let body = b"Hello, World!";
        let request = Request::new(
            Method::Get,
            "/".into(),
            body.len(),
            Cursor::new(body.to_vec()),
        );

        stream.write_message(request.clone()).unwrap();
        stream.set_position(0);

        let header: Header<_> = stream.read_message().unwrap();
        let mut actual: Request<_> = header.read_message().unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(request, actual);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
