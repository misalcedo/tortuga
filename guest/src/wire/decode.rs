use std::io::{self, ErrorKind, Read};

use crate::{Frame, FrameType, Method, Uri};

pub trait Decode<Value> {
    fn decode(&mut self) -> io::Result<Value>;
}

impl<R> Decode<u8> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<u8> {
        let mut value = 0u8.to_le_bytes();
        self.read_exact(&mut value)?;
        Ok(u8::from_le_bytes(value))
    }
}

impl<R> Decode<u16> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<u16> {
        let mut value = 0u16.to_le_bytes();
        self.read_exact(&mut value)?;
        Ok(u16::from_le_bytes(value))
    }
}

impl<R> Decode<u64> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<u64> {
        let mut value = 0u64.to_le_bytes();
        self.read_exact(&mut value)?;
        Ok(u64::from_le_bytes(value))
    }
}

impl<R> Decode<usize> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<usize> {
        let value: u64 = self.decode()?;

        Ok(value as usize)
    }
}

impl<R> Decode<String> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<String> {
        let length: usize = self.decode()?;
        let mut value = vec![0; length];

        self.read_exact(&mut value)?;

        String::from_utf8(value).map_err(|_| ErrorKind::InvalidData.into())
    }
}

impl<R> Decode<Uri> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<Uri> {
        let value: String = self.decode()?;

        Ok(Uri::from(value))
    }
}

impl<R> Decode<FrameType> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<FrameType> {
        let kind: u8 = self.decode()?;

        FrameType::try_from(kind).map_err(|_| ErrorKind::InvalidData.into())
    }
}

impl<R> Decode<Frame> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<Frame> {
        let kind = self.decode()?;
        let length = self.decode()?;

        Ok(Frame::new(kind, length))
    }
}

impl<R> Decode<Method> for R
where
    R: Read,
{
    fn decode(&mut self) -> io::Result<Method> {
        let method: u8 = self.decode()?;

        Method::try_from(method).map_err(|_| ErrorKind::InvalidData.into())
    }
}
