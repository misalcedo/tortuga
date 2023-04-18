use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Cursor, ErrorKind, Read, Write};

use crate::frame::FrameType;
use crate::wire::{Decode, Encode};
use crate::{Frame, IoLimiter};

pub trait Body: Read {
    fn len(&mut self) -> Option<usize> {
        None
    }
}

impl<R> Body for FrameIo<R>
where
    R: Read,
{
    fn len(&mut self) -> Option<usize> {
        Some(self.length)
    }
}

impl Body for Cursor<Vec<u8>> {
    fn len(&mut self) -> Option<usize> {
        Some(self.get_ref().len())
    }
}

impl Body for File {
    fn len(&mut self) -> Option<usize> {
        Some(self.metadata().ok()?.len() as usize)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FrameIo<R> {
    length: usize,
    io: IoLimiter<R>,
}

impl<IO> FrameIo<IO> {
    pub fn new(io: IO, length: usize) -> Self {
        FrameIo {
            length,
            io: IoLimiter::new(io, 0),
        }
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn resize(&mut self, length: usize) {
        self.length = length;
    }

    pub fn get_mut(&mut self) -> &mut IO {
        self.io.get_mut()
    }

    pub fn finish(self) -> IO {
        self.io.finish()
    }
}

impl<R> Read for FrameIo<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.length == 0 {
            return Ok(0);
        }

        if self.io.is_empty() && !self.is_empty() {
            let frame: Frame = self.io.get_mut().decode()?;

            if frame.kind() != FrameType::Data || frame.len() > self.length {
                return Err(ErrorKind::InvalidData.into());
            }

            self.io.resize(frame.len());
        }

        let bytes_read = self.io.read(buf)?;

        self.length -= bytes_read;

        Ok(bytes_read)
    }
}

impl<W> Write for FrameIo<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.length == 0 {
            return Ok(0);
        }

        let frame = Frame::new(FrameType::Data, buf.len());

        self.io.get_mut().encode(frame)?;
        self.io.get_mut().write_all(buf)?;
        self.length -= buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn transfer() {
        let body = Vec::from("Hello, World!");
        let mut stream = FrameIo::new(Cursor::new(Vec::new()), body.len());

        stream.write_all(body.as_slice()).unwrap();
        stream.get_mut().set_position(0);
        stream.resize(body.len());

        let mut actual = vec![0; body.len()];

        stream.read_exact(actual.as_mut_slice()).unwrap();

        assert_eq!(body.as_slice(), actual.as_slice());
    }
}
