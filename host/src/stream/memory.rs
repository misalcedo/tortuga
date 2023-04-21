use std::io::{self, Cursor, Read, Write};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tortuga_guest::{Destination, Source};

use tortuga_guest::wire::{ReadableMessage, WritableMessage};

use crate::wasm;

#[derive(Clone, Debug, Default)]
pub struct Stream {
    reader: Arc<Mutex<Vec<u8>>>,
    writer: Arc<Mutex<Vec<u8>>>,
    cursor: u64,
}

impl Stream {
    fn new() -> (Self, Self) {
        let reader = Self::default();
        let mut writer = reader.clone();

        std::mem::swap(&mut writer.reader, &mut writer.writer);

        (reader, writer)
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut guard = match self.reader.lock() {
            Ok(reader) => reader,
            Err(e) => e.into_inner(),
        };

        let mut cursor = Cursor::new(guard.as_mut_slice());

        cursor.set_position(self.cursor);

        let result = cursor.read(buf);

        self.cursor = cursor.position();

        result
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut guard = match self.writer.lock() {
            Ok(writer) => writer,
            Err(e) => e.into_inner(),
        };

        guard.extend_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl wasm::Stream for Stream {
    type Error = io::Error;

    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        Read::read(self, buffer)
    }

    async fn write(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        Write::write(self, buffer)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Factory {
    streams: Arc<Mutex<Vec<Stream>>>,
}

impl Factory {
    pub fn read_message<Message>(&mut self, index: usize) -> io::Result<Message>
    where
        Message: ReadableMessage<Stream>,
    {
        let mut guard = match self.streams.lock() {
            Ok(streams) => streams,
            Err(e) => e.into_inner(),
        };

        let stream = guard
            .get_mut(index)
            .ok_or_else(|| io::Error::from(io::ErrorKind::ConnectionReset))?;

        let source = std::mem::take(stream);

        source.read_message()
    }

    pub fn write_message<Message>(&mut self, index: usize, message: Message) -> io::Result<usize>
    where
        Message: WritableMessage,
    {
        let mut guard = match self.streams.lock() {
            Ok(streams) => streams,
            Err(e) => e.into_inner(),
        };

        let stream = guard
            .get_mut(index)
            .ok_or_else(|| io::Error::from(io::ErrorKind::ConnectionReset))?;

        stream.write_message(message)
    }
}

impl wasm::Factory<Stream> for Factory {
    fn create(&mut self) -> Stream {
        let (a, b) = Stream::new();

        let mut guard = match self.streams.lock() {
            Ok(streams) => streams,
            Err(e) => e.into_inner(),
        };

        guard.push(b);

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::Factory;
    use tortuga_guest::{Method, Request};

    #[test]
    fn client_server() {
        let (mut client, mut server) = Stream::new();
        let mut buffer = Cursor::new(Vec::new());
        let body = b"Hello, World!";

        assert_eq!(body.len(), client.write(body).unwrap());

        std::io::copy(&mut server, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    #[test]
    fn factory() {
        let mut factory = super::Factory::default();
        let mut client = factory.create();
        let mut buffer = Cursor::new(Vec::new());

        let body = b"Hello, World!";
        let request = Request::new(Method::Get, "/", Cursor::new(body.to_vec()));

        client.write_message(request).unwrap();

        let mut actual: Request<_> = factory.read_message(0).unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
