use crate::wire::Decode;
use crate::{Frame, FrameIo, FrameType, Request, Response, Source};
use std::io::{self, ErrorKind, Read};

pub trait ReadableMessage: Sized {
    type Body: Read;

    fn read_from(reader: Self::Body) -> io::Result<Self>;
}

impl<Body> ReadableMessage for Request<Option<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(mut reader: Self::Body) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let method = reader.decode()?;
        let uri = reader.decode()?;
        let length = reader.decode()?;

        Ok(Request::new(method, uri, length, Default::default()))
    }
}

impl<Body> ReadableMessage for Request<FrameIo<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(reader: Self::Body) -> io::Result<Self> {
        let mut header: Request<Option<Self::Body>> = reader.read_message()?;
        let body = FrameIo::new(
            header
                .body()
                .take()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?,
            header.content_length(),
        );

        Ok(header.with_body(body))
    }
}

impl<Body> ReadableMessage for Response<Option<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(mut reader: Self::Body) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let status: u16 = reader.decode()?;
        let length = reader.decode()?;

        Ok(Response::new(status, length, Default::default()))
    }
}

impl<Body> ReadableMessage for Response<FrameIo<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(reader: Self::Body) -> io::Result<Self> {
        let mut header: Response<Option<Self::Body>> = reader.read_message()?;
        let body = FrameIo::new(
            header
                .body()
                .take()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?,
            header.content_length(),
        );
        Ok(header.with_body(body))
    }
}
