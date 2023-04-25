use crate::frame::Header;
use crate::wire::Decode;
use crate::{Frame, FrameIo, FrameType, Request, Response, Source};
use std::io::{self, Read};

pub trait ReadableMessage: Sized {
    type Body: Read;

    fn read_from(reader: Self::Body) -> io::Result<Self>;
}

impl<Body> ReadableMessage for Header<Body>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(mut reader: Self::Body) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(io::ErrorKind::InvalidData.into());
        }

        Ok(Header::new(reader, header.len()))
    }
}

impl<Body> ReadableMessage for Request<FrameIo<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(reader: Self::Body) -> io::Result<Self> {
        let mut header: Header<_> = reader.read_message()?;

        let method = header.decode()?;
        let uri = header.decode()?;
        let length = header.decode()?;
        let body = FrameIo::new(header.finish().1, length);

        Ok(Request::new(method, uri, length, body))
    }
}

impl<Body> ReadableMessage for Response<FrameIo<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(reader: Self::Body) -> io::Result<Self> {
        let mut header: Header<_> = reader.read_message()?;

        let status: u16 = header.decode()?;
        let length = header.decode()?;
        let body = FrameIo::new(header.finish().1, length);

        Ok(Response::new(status, length, body))
    }
}
