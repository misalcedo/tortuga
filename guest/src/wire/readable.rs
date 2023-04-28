use crate::frame::Header;
use crate::wire::{Decode, Encode};
use crate::{Frame, FrameIo, FrameType, Request, Response};
use std::io::{self, Read};

pub trait ReadableMessage: Sized {
    type Body: Read;

    fn read_from(reader: Self::Body) -> io::Result<Self>;
}

fn read_header_frame<Body>(reader: &mut Body) -> io::Result<Frame>
where
    Body: Read,
{
    let frame: Frame = reader.decode()?;

    if frame.kind() != FrameType::Header {
        return Err(io::ErrorKind::InvalidData.into());
    }

    Ok(frame)
}

impl<Body> ReadableMessage for Header<Body>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(mut reader: Self::Body) -> io::Result<Self> {
        let frame = read_header_frame(&mut reader)?;

        let mut header = vec![0; Frame::bytes() + frame.len()];

        header.as_mut_slice().as_mut().encode(frame)?;
        reader.read_exact(&mut header[Frame::bytes()..])?;

        Ok(Header::new(header, reader))
    }
}

impl<Body> ReadableMessage for Request<FrameIo<Body>>
where
    Body: Read,
{
    type Body = Body;

    fn read_from(mut reader: Self::Body) -> io::Result<Self> {
        read_header_frame(&mut reader)?;

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
    type Body = Body;

    fn read_from(mut reader: Self::Body) -> io::Result<Self> {
        read_header_frame(&mut reader)?;

        let status: u16 = reader.decode()?;
        let length = reader.decode()?;
        let body = FrameIo::new(reader, length);

        Ok(Response::new(status, length, body))
    }
}
