use crate::{Frame, FrameType, Method, Uri};
use std::io::{self, Write};

pub trait Encode<Value> {
    fn encode(&mut self, value: Value) -> io::Result<usize>;
}

impl<W> Encode<u8> for W
where
    W: Write,
{
    fn encode(&mut self, value: u8) -> io::Result<usize> {
        let buffer = value.to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<u16> for W
where
    W: Write,
{
    fn encode(&mut self, value: u16) -> io::Result<usize> {
        let buffer = value.to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<u64> for W
where
    W: Write,
{
    fn encode(&mut self, value: u64) -> io::Result<usize> {
        let buffer = value.to_le_bytes();

        self.write_all(&buffer)?;

        Ok(buffer.len())
    }
}

impl<W> Encode<usize> for W
where
    W: Write,
{
    fn encode(&mut self, value: usize) -> io::Result<usize> {
        self.encode(value as u64)
    }
}

impl<W> Encode<String> for W
where
    W: Write,
{
    fn encode(&mut self, value: String) -> io::Result<usize> {
        self.encode(value.as_str())
    }
}

impl<W> Encode<&str> for W
where
    W: Write,
{
    fn encode(&mut self, value: &str) -> io::Result<usize> {
        let mut bytes = self.encode(value.len())?;
        let buffer = value.as_bytes();

        self.write_all(buffer)?;

        bytes += buffer.len();

        Ok(bytes)
    }
}

impl<W> Encode<Uri> for W
where
    W: Write,
{
    fn encode(&mut self, value: Uri) -> io::Result<usize> {
        self.encode(value.as_ref())
    }
}

impl<W> Encode<&Uri> for W
where
    W: Write,
{
    fn encode(&mut self, value: &Uri) -> io::Result<usize> {
        self.encode(value.as_ref())
    }
}

impl<W> Encode<FrameType> for W
where
    W: Write,
{
    fn encode(&mut self, value: FrameType) -> io::Result<usize> {
        self.encode(value as u8)
    }
}

impl<W> Encode<Frame> for W
where
    W: Write,
{
    fn encode(&mut self, value: Frame) -> io::Result<usize> {
        let mut bytes = 0;

        bytes += self.encode(value.kind())?;
        bytes += self.encode(value.len())?;

        Ok(bytes)
    }
}

impl<W> Encode<Method> for W
where
    W: Write,
{
    fn encode(&mut self, value: Method) -> io::Result<usize> {
        self.encode(value as u8)
    }
}
