use crate::wasm;
use crate::wasm::Stream;
use async_trait::async_trait;
use std::io::{self, Cursor, Read};
use tortuga_guest::wire::{ReadableMessage, WritableMessage};
use tortuga_guest::{Body, Destination, FrameIo, Request, Response, Source};

pub struct PeekingStream<Stream> {
    buffer: Cursor<Vec<u8>>,
    stream: Stream,
}

impl<Stream> From<Stream> for PeekingStream<Stream>
where
    Stream: wasm::Stream,
{
    fn from(stream: Stream) -> Self {
        PeekingStream {
            buffer: Default::default(),
            stream,
        }
    }
}

impl<Stream> ReadableMessage<PeekingStream<Stream>> for Request<FrameIo<PeekingStream<Stream>>> {
    fn read_from(reader: PeekingStream<Stream>) -> io::Result<Self> {
        todo!()
    }

    fn finish(self) -> PeekingStream<Stream> {
        self.into_body().finish()
    }
}

impl<Stream> ReadableMessage<PeekingStream<Stream>> for Response<FrameIo<PeekingStream<Stream>>>
where
    Stream: Source + Body,
{
    fn read_from(mut reader: PeekingStream<Stream>) -> io::Result<Self> {
        let mut message: Response<_> = reader.stream.read_message()?;

        message.write_header_to(&mut reader.buffer)?;

        Ok(message)
    }

    fn finish(self) -> PeekingStream<Stream> {
        self.into_body().finish()
    }
}

impl<Stream> Source for PeekingStream<Stream>
where
    Stream: Source,
{
    fn read_message<M>(self) -> io::Result<M>
    where
        M: ReadableMessage<Self>,
    {
        self.stream.read_message()
    }
}

impl<Stream> Destination for PeekingStream<Stream>
where
    Stream: Destination,
{
    fn write_message<M>(&mut self, message: M) -> std::io::Result<usize>
    where
        M: WritableMessage,
    {
        self.stream.write_message(message)
    }
}

#[async_trait]
impl<Stream> wasm::Stream for PeekingStream<Stream>
where
    Stream: wasm::Stream,
{
    type Error = Stream::Error;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        if self.buffer.position() as usize == self.buffer.get_ref().len() {
            self.stream.read(buffer).await
        } else {
            match self.buffer.read(buffer) {
                Ok(bytes) => Ok(bytes),
                Err(_) => Ok(0),
            }
        }
    }

    async fn write(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.stream.write(buffer).await
    }
}
