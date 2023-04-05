use std::io::{self, Cursor, Seek, SeekFrom, Write};
use crate::{Body, Frame, FrameIo, FrameType, Method, Request, Response};

pub trait Encode<Value> {
    fn encode(&mut self, value: Value) -> io::Result<usize>;
}

impl<W> Encode<u8> for W where W: Write {
    fn encode(&mut self, value: u8) -> io::Result<usize> {
        let buffer = value.to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<u16> for W where W: Write {
    fn encode(&mut self, value: u16) -> io::Result<usize> {
        let buffer = value.to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<u64> for W where W: Write {
    fn encode(&mut self, value: u64) -> io::Result<usize> {
        let buffer = value.to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<usize> for W where W: Write {
    fn encode(&mut self, value: usize) -> io::Result<usize> {
        let buffer = (value as u64).to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<String> for W where W: Write {
    fn encode(&mut self, value: String) -> io::Result<usize> {
        self.encode(value.as_str())
    }
}

impl<W> Encode<&str> for W where W: Write {
    fn encode(&mut self, value: &str) -> io::Result<usize> {
        let mut bytes = self.encode(value.len())?;
        let buffer = value.as_bytes();

        self.write_all(buffer)?;

        bytes += buffer.len();

        Ok(bytes)
    }
}

impl<W> Encode<FrameType> for W where W: Write {
    fn encode(&mut self, value: FrameType) -> io::Result<usize> {
        self.encode(value as u8)
    }
}

impl<W> Encode<Frame> for W where W: Write {
    fn encode(&mut self, value: Frame) -> io::Result<usize> {
        let mut bytes = 0;

        bytes += self.encode(value.kind())?;
        bytes += self.encode(value.len())?;

        Ok(bytes)
    }
}

impl<W> Encode<Method> for W where W: Write {
    fn encode(&mut self, value: Method) -> io::Result<usize> {
        self.encode(value as u8)
    }
}

pub trait WritableMessage {
    fn write_to<W>(self, writer: &mut W) -> io::Result<usize> where W: Write;
}

impl<B> WritableMessage for Request<B> where B: Body {
    fn write_to<W>(mut self, writer: &mut W) -> io::Result<usize> where W: Write {
        let length = self.body().stream_position()? as usize;
        let mut buffer = Cursor::new(Vec::with_capacity(64));

        buffer.encode(self.method() as u8)?;
        buffer.encode(self.uri())?;
        buffer.encode(length)?;

        let header = Frame::new(FrameType::Header, buffer.stream_position()? as usize);
        let mut bytes = 0;

        bytes += writer.encode(header)?;

        self.body().seek(SeekFrom::Start(0))?;

        let mut body = FrameIo::new(writer, length);

        bytes += io::copy(self.body(), &mut body)? as usize;

        Ok(bytes)
    }
}

impl<B> WritableMessage for Response<B> where B: Body {
    fn write_to<W>(mut self, writer: &mut W) -> io::Result<usize> where W: Write {
        let length = self.body().stream_position()? as usize;
        let mut buffer = Cursor::new(Vec::with_capacity(64));

        buffer.encode(self.status())?;
        buffer.encode(length)?;

        let header = Frame::new(FrameType::Header, buffer.stream_position()? as usize);
        let mut bytes = 0;

        bytes += writer.encode(header)?;

        self.body().seek(SeekFrom::Start(0))?;

        let mut body = FrameIo::new(writer, length);

        bytes += io::copy(self.body(), &mut body)? as usize;

        Ok(bytes)
    }
}
