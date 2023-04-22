use crate::wire::Decode;
use crate::{Frame, FrameIo, FrameType, Request, Response};
use std::io::{self, ErrorKind, Read};

pub trait ReadableMessage<R>: Sized
where
    R: ?Sized,
{
    fn read_from(reader: R) -> io::Result<Self>;
    fn finish(self) -> R;
}

impl<R> ReadableMessage<R> for Request<FrameIo<R>>
where
    R: Read,
{
    fn read_from(mut reader: R) -> io::Result<Self> {
        let header: Frame = reader.decode()?;

        if header.kind() != FrameType::Header {
            return Err(ErrorKind::InvalidData.into());
        }

        let method = reader.decode()?;
        let uri: String = reader.decode()?;
        let length = reader.decode()?;
        let body = FrameIo::new(reader, length);

        Ok(Request::new(method, uri, body))
    }

    fn finish(self) -> R {
        self.into_body().finish()
    }
}

impl<R> ReadableMessage<R> for Response<FrameIo<R>>
where
    R: Read,
{
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
