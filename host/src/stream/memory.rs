use std::collections::VecDeque;
use std::future::Future;
use std::io::{self, Cursor, Read, Write};
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;
use tortuga_guest::{Destination, Source};

use tortuga_guest::wire::{ReadableMessage, WritableMessage};

use crate::{executor, wasm};

#[derive(Clone, Debug, Default)]
pub struct Stream {
    reader: Arc<Mutex<Vec<u8>>>,
    writer: Arc<Mutex<Vec<u8>>>,
    read_waker: Arc<Mutex<Option<Waker>>>,
    write_waker: Arc<Mutex<Option<Waker>>>,
    cursor: u64,
}

impl Stream {
    fn new() -> (Self, Self) {
        let left = Self::default();
        let right = Stream {
            reader: left.writer.clone(),
            writer: left.reader.clone(),
            read_waker: left.write_waker.clone(),
            write_waker: left.read_waker.clone(),
            cursor: 0,
        };

        (left, right)
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
        let mut waker_guard = match self.write_waker.lock() {
            Ok(waker) => waker,
            Err(e) => e.into_inner(),
        };

        guard.extend_from_slice(buf);

        if let Some(waker) = waker_guard.take() {
            waker.wake()
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Future for Stream {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let read_guard = match self.reader.lock() {
            Ok(reader) => reader,
            Err(e) => e.into_inner(),
        };
        let mut waker_guard = match self.read_waker.lock() {
            Ok(waker) => waker,
            Err(e) => e.into_inner(),
        };

        if self.cursor >= read_guard.len() as u64 {
            *waker_guard = Some(cx.waker().clone());

            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

#[async_trait]
impl wasm::Stream for Stream {
    type Error = io::Error;

    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut bytes = Read::read(self, buffer)?;

        if bytes == 0 {
            self.clone().await;
            bytes += Read::read(self, buffer)?;
        }

        Ok(bytes)
    }

    async fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        Write::write(self, buffer)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Bridge {
    streams: Arc<Mutex<VecDeque<Stream>>>,
    waker: Option<Waker>,
}

impl Bridge {
    pub fn read_message<Message>(&mut self, index: usize) -> io::Result<Message>
    where
        Message: ReadableMessage<Body = Stream>,
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

    fn pop(&mut self) -> Option<Stream> {
        let mut guard = match self.streams.lock() {
            Ok(streams) => streams,
            Err(e) => e.into_inner(),
        };

        guard.pop_front()
    }
}

impl wasm::Factory for Bridge {
    type Stream = Stream;

    fn create(&mut self) -> Self::Stream {
        let (a, b) = Stream::new();

        let mut guard = match self.streams.lock() {
            Ok(streams) => streams,
            Err(e) => e.into_inner(),
        };

        guard.push_back(b);

        if let Some(waker) = self.waker.take() {
            waker.wake();
        }

        a
    }
}

impl<'a> Future for Bridge {
    type Output = Stream;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.deref_mut().pop() {
            None => {
                self.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            Some(stream) => Poll::Ready(stream),
        }
    }
}

#[async_trait]
impl executor::Provider for Bridge {
    type Stream = Stream;

    fn try_next(&mut self) -> Option<Self::Stream> {
        self.pop()
    }

    async fn next(&mut self) -> Self::Stream {
        loop {
            if let Some(stream) = self.pop() {
                return stream;
            }
        }
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
        let mut factory = Bridge::default();
        let mut client = factory.create();
        let mut buffer = Cursor::new(Vec::new());

        let body = b"Hello, World!";
        let request = Request::new(
            Method::Get,
            "/".into(),
            body.len(),
            Cursor::new(body.to_vec()),
        );

        client.write_message(request).unwrap();

        let mut actual: Request<_> = factory.read_message(0).unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), body);
    }
}
