use std::future::Future;
use std::io::{Cursor, Read, Write};
use std::net::Shutdown;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;

use tortuga_model::asynchronous;

pub struct Pipe {
    buffer: Vec<u8>,
    read_cursor: usize,
    write_cursor: usize,
    length: usize,
    waker: Option<Waker>,
}

impl Pipe {
    pub fn new(capacity: usize) -> Self {
        Pipe {
            buffer: vec![0u8; capacity],
            read_cursor: 0,
            write_cursor: 0,
            length: 0,
            waker: None,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let end = self.buffer.len();
        let bytes = self.length.min(buffer.len());

        for byte in 0..bytes {
            buffer[byte] = self.buffer[self.read_cursor];
            self.read_cursor = (self.read_cursor + 1) % end;
        }

        self.length -= bytes;

        bytes
    }

    pub fn write(&mut self, buffer: &[u8]) -> usize {
        let end = self.buffer.len();
        let remaining = end - self.length;
        let bytes = remaining.min(buffer.len());

        for byte in 0..bytes {
            self.buffer[self.write_cursor] = buffer[byte];
            self.write_cursor = (self.write_cursor + 1) % end;
        }

        self.length += bytes;

        bytes
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.length
    }
}

pub struct WriteFuture(Pipe);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {}

    #[test]
    fn empty() {
        let pipe = Pipe::new(0);
    }
}
