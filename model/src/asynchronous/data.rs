use crate::asynchronous::encoding::{Deserialize, Serialize};
use crate::asynchronous::Encoding;
use crate::frame;
use crate::{asynchronous, Frame};
use async_trait::async_trait;
use std::io;

pub trait FrameEncoding:
    Serialize<frame::Data, Error = Self::Error>
    + Deserialize<frame::Data, Error = Self::Error>
    + Serialize<[u8], Error = Self::Error>
    + Deserialize<Vec<u8>, Error = Self::Error>
    + Send
    + Sync
{
    type Error;
}

pub struct FrameWire<Encoding, Wire> {
    encoding: Encoding,
    wire: Wire,
    remaining: usize,
}

impl<Encoding, Wire> FrameWire<Encoding, Wire> {
    pub fn into_inner(self) -> Wire {
        self.wire
    }
}

impl<Encoding, Error, Wire> FrameWire<Encoding, Wire>
where
    Encoding: FrameEncoding,
    Error: Encoding::Error,
    Wire: asynchronous::Wire,
{
    fn new(encoding: Encoding, wire: Wire) -> Self {
        FrameWire {
            encoding,
            wire,
            remaining: 0,
        }
    }
}

#[async_trait]
impl<Encoding, Error, Wire> asynchronous::Wire for FrameWire<Encoding, Wire>
where
    Encoding: FrameEncoding,
    Error: Encoding::Error,
    Wire: asynchronous::Wire,
{
    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            let frame: frame::Data = self
                .encoding
                .deserialize(&mut self.wire)
                .await
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

            self.remaining = frame.len();
        }

        let length = self.remaining.min(buffer.len());

        self.wire.read(&mut buffer[..length]).await
    }

    async fn read_exact(&mut self, mut buffer: &mut [u8]) -> io::Result<()> {
        let mut bytes = self.read(buffer).await?;
        let mut view = &mut buffer[..];

        while !view.is_empty() {
            let current = self.read(view).await?;

            if current == 0 {
                break;
            }

            bytes += current;
            view = &mut view[current..];
        }

        if view.is_empty() {
            Ok(())
        } else {
            Err(io::Error::from(io::ErrorKind::UnexpectedEof))
        }
    }

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let frame = frame::Data::from(buffer.len());
        let bytes = self
            .encoding
            .serialize(&frame, &mut self.wire)
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;

        self.encoding
            .serialize(buffer, &mut self.wire)
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;

        Ok(bytes + buffer.len())
    }

    async fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
        self.write(buffer).await.map(|_| ())
    }
}
