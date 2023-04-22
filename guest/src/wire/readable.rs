use crate::wire::Decode;
use crate::{Frame, FrameIo, FrameType, Request, Response};
use std::io::{self, ErrorKind, Read};

pub trait ReadableMessage: Sized {
    type Header;
    type Body: Read;

    fn read_header<R>(reader: &mut R) -> io::Result<Self::Header>
    where
        R: Read;

    fn read_from(reader: Self::Body) -> io::Result<Self>;
}

impl<Body> ReadableMessage for Request<FrameIo<Body>>
where
    Body: Read,
{
    type Header = Request<()>;
    type Body = Body;

    fn read_header<R>(reader: &mut R) -> io::Result<Self::Header>
    where
        R: Read,
    {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let method = reader.decode()?;
        let uri = reader.decode()?;
        let length = reader.decode()?;

        Ok(Request::empty(method, uri, length))
    }

    fn read_from(reader: Self::Body) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let method = reader.decode()?;
        let uri = reader.decode()?;
        let length = reader.decode()?;
        let body = FrameIo::new(reader, length);

        Ok(Request::new(method, uri, length, body))
    }
}

impl<Body> ReadableMessage for Response<FrameIo<Body>>
where
    Body: Read,
{
    type Header = Response<()>;
    type Body = Body;

    fn read_header<R>(reader: &mut R) -> io::Result<Self::Header>
    where
        R: Read,
    {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let status: u16 = reader.decode()?;
        let length = reader.decode()?;

        Ok(Response::empty(status, length))
    }

    fn read_from(reader: Self::Body) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let status: u16 = reader.decode()?;
        let length = reader.decode()?;
        let body = FrameIo::new(reader, length);

        Ok(Response::new(status, length, body))
    }
}
