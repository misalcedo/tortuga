use crate::frame::FrameType;
use crate::wire::{Decode, Encode};
use crate::{Frame, IoLimiter};
use std::io::{self, ErrorKind, Read, Seek, Write};

pub trait Body: Read + Seek {}

impl<B: Read + Seek> Body for B {}

#[derive(Debug)]
pub struct FrameIo<R> {
    length: usize,
    remaining: usize,
    io: IoLimiter<R>,
}

impl<IO> FrameIo<IO> {
    pub fn new(io: IO, length: usize) -> Self {
        FrameIo {
            length,
            remaining: length,
            io: IoLimiter::new(io, 0),
        }
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
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
        if self.io.is_empty() && self.remaining > 0 {
            let frame: Frame = self.io.get_mut().decode()?;

            if frame.kind() != FrameType::Data || frame.len() > self.remaining {
                return Err(ErrorKind::InvalidData.into());
            }

            self.io.resize(frame.len());
        }

        let bytes_read = self.io.read(buf)?;

        self.remaining -= bytes_read;

        Ok(bytes_read)
    }
}

impl<W> Write for FrameIo<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Ok(0);
        }

        let frame = Frame::new(FrameType::Data, buf.len());

        self.io.get_mut().encode(frame)?;
        self.io.get_mut().write_all(buf)?;
        self.remaining -= buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}
