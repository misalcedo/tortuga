use std::io::{self, ErrorKind, Read};

use crate::{Frame, FrameIo, FrameType, Method, Request, Response};

pub trait Decode<Value> {
    fn decode(&mut self) -> io::Result<Value>;
}

impl<R> Decode<u8> for R where R: Read {
    fn decode(&mut self) -> io::Result<u8> {
        let mut value = [0; 1];
        self.read_exact(&mut value)?;
        Ok(u8::from_le_bytes(value))
    }
}

impl<R> Decode<u16> for R where R: Read {
    fn decode(&mut self) -> io::Result<u16> {
        let mut value = [0; 2];
        self.read_exact(&mut value)?;
        Ok(u16::from_le_bytes(value))
    }
}

impl<R> Decode<u64> for R where R: Read {
    fn decode(&mut self) -> io::Result<u64> {
        let mut value = [0; 8];
        self.read_exact(&mut value)?;
        Ok(u64::from_le_bytes(value))
    }
}

impl<R> Decode<usize> for R where R: Read {
    fn decode(&mut self) -> io::Result<usize> {
        let value: u64 = self.decode()?;

        Ok(value as usize)
    }
}

impl<R> Decode<String> for R where R: Read {
    fn decode(&mut self) -> io::Result<String> {
        let length: usize = self.decode()?;
        let mut value = vec![0; length as usize];

        self.read_exact(&mut value)?;

        String::from_utf8(value).map_err(|_| ErrorKind::InvalidData.into())
    }
}

impl<R> Decode<FrameType> for R where R: Read {
    fn decode(&mut self) -> io::Result<FrameType> {
        let kind: u8 = self.decode()?;

        FrameType::try_from(kind).map_err(|_| ErrorKind::InvalidData.into())
    }
}

impl<R> Decode<Frame> for R where R: Read {
    fn decode(&mut self) -> io::Result<Frame> {
        let kind = self.decode()?;
        let length = self.decode()?;

        Ok(Frame::new(kind, length))
    }
}

impl<R> Decode<Method> for R where R: Read {
    fn decode(&mut self) -> io::Result<Method> {
        let method: u8 = self.decode()?;

        Method::try_from(method).map_err(|_| ErrorKind::InvalidData.into())
    }
}

pub trait ReadableMessage<R>: Sized where R: ?Sized {
    fn read_from(reader: R) -> io::Result<Self>;
    fn finish(self) -> R;
}

impl<R> ReadableMessage<R> for Request<FrameIo<R>> where R: Read {
    fn read_from(mut reader: R) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let method = reader.decode()?;
        let uri = reader.decode()?;
        let length = reader.decode()?;
        let body = FrameIo::new(reader, length);

        Ok(Request::new(method, uri, body))
    }

    fn finish(self) -> R {
        self.consume_body().finish()
    }
}

impl<R> ReadableMessage<R> for Response<FrameIo<R>> where R: Read {
    fn read_from(mut reader: R) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let status: u16 = reader.decode()?;
        let length = reader.decode()?;
        let body = FrameIo::new(reader, length);

        Ok(Response::new(status, body))
    }

    fn finish(self) -> R {
        self.consume_body().finish()
    }
}
