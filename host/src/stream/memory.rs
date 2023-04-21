use std::convert::Infallible;
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tortuga_guest::{Destination, Source};

use tortuga_guest::wire::{ReadableMessage, WritableMessage};

use crate::wasm;

#[derive(Clone, Debug, Default)]
pub struct Stream {
    reader: Arc<Mutex<Cursor<Vec<u8>>>>,
    writer: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl Stream {
    fn new() -> (Self, Self) {
        let reader = Self::default();
        let writer = reader.swapped();

        (reader, writer)
    }

    fn reset(&mut self) {
        match self.reader.lock() {
            Ok(mut reader) => reader.set_position(0),
            Err(mut e) => e.get_mut().set_position(0),
        }

        match self.writer.lock() {
            Ok(mut writer) => writer.set_position(0),
            Err(mut e) => e.get_mut().set_position(0),
        }
    }

    fn swapped(&self) -> Self {
        let mut clone = self.clone();
        std::mem::swap(&mut clone.reader, &mut clone.writer);
        clone
    }
}

impl Seek for Stream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let mut guard = match self.reader.lock() {
            Ok(reader) => reader,
            Err(e) => e.into_inner(),
        };

        guard.seek(pos)
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.reader.lock() {
            Ok(mut reader) => reader.read(buf),
            Err(mut e) => e.get_mut().read(buf),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.writer.lock() {
            Ok(mut writer) => writer.write(buf),
            Err(mut e) => e.get_mut().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl wasm::Stream for Stream {
    type Error = Infallible;

    async fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        Read::read(self, buffer)
    }

    async fn write(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
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
            Ok(mut streams) => streams,
            Err(mut e) => e.into_inner(),
        };

        let mut stream = guard
            .get_mut(index)
            .ok_or_else(|| io::Error::from(io::ErrorKind::ConnectionReset))?;

        let source = std::mem::replace(stream, Stream::default());

        source.read_message()
    }

    pub fn write_message<Message>(&mut self, index: usize, message: Message) -> io::Result<usize>
    where
        Message: WritableMessage,
    {
        let mut guard = match self.streams.lock() {
            Ok(mut streams) => streams,
            Err(mut e) => e.into_inner(),
        };

        let mut stream = guard
            .get_mut(index)
            .ok_or_else(|| io::Error::from(io::ErrorKind::ConnectionReset))?;

        stream.write_message(message)
    }
}

impl wasm::Factory<Stream> for Factory {
    fn create_primary(&mut self) -> Stream {
        Stream::default()
    }

    fn create(&mut self) -> Stream {
        let (a, b) = Stream::new();

        let mut guard = match self.streams.lock() {
            Ok(mut streams) => streams,
            Err(mut e) => e.into_inner(),
        };

        guard.push(b);

        a
    }
}
