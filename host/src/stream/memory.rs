use std::convert::Infallible;
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use tortuga_guest::wire::{ReadableMessage, WritableMessage};

use crate::wasm;

#[derive(Clone, Debug, Default)]
pub struct Stream {
    client: Arc<Mutex<Cursor<Vec<u8>>>>,
    server: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl Stream {
    fn new() -> (Self, Self) {
        let client = Self::default();
        let server = client.swapped();

        (client, server)
    }

    fn swapped(&self) -> Self {
        let mut clone = self.clone();
        std::mem::swap(&mut clone.client, &mut clone.server);
        clone
    }

    fn bytes(&self) -> Vec<u8> {
        match self.client.lock() {
            Ok(client) => client.get_ref().clone(),
            Err(mut e) => {
                e.get_mut().get_mut().clear();
                e.get_ref().get_ref().clone()
            }
        }
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

#[async_trait]
impl wasm::Stream for Stream {
    type Error = Infallible;

    async fn read(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    async fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        todo!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Factory {
    streams: Arc<Mutex<Vec<Stream>>>,
}

impl Factory {
    fn len(&self) -> usize {
        match self.streams.lock() {
            Ok(streams) => streams.len(),
            Err(mut e) => {
                e.get_mut().clear();
                0
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self.streams.lock() {
            Ok(streams) => streams.is_empty(),
            Err(mut e) => {
                e.get_mut().clear();
                true
            }
        }
    }

    fn get(&self, index: usize) -> Option<Vec<u8>> {
        match self.streams.lock() {
            Ok(streams) => streams.get(index).map(Stream::bytes),
            Err(mut e) => {
                e.get_mut().clear();
                e.get_ref().get(index).map(Stream::bytes)
            }
        }
    }
}

impl wasm::Factory<Stream> for Factory {
    fn create(&mut self) -> Stream {
        let (a, b) = Stream::new();

        match self.streams.lock() {
            Ok(mut streams) => streams.push(b),
            Err(mut e) => {
                e.get_mut().clear();
                e.get_mut().push(b);
            }
        }

        a
    }
}
