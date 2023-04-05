use std::io::{ErrorKind, Read, Seek, Write};
use crate::{Frame, IoLimiter};
use crate::frame::FrameType;
use crate::wire::Decode;

pub trait Body: Read + Seek {}

impl<B: Read + Seek> Body for B {}

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
            io: IoLimiter::new(io, Frame::bytes())
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

impl<R> Read for FrameIo<R> where R: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.io.is_empty() && !self.is_empty() {
            let frame: Frame = self.io.decode()?;

            if frame.kind() != FrameType::Data {
                return Err(ErrorKind::InvalidData.into());
            }

            if frame.len() > self.remaining {
                return Err(ErrorKind::InvalidData.into());
            }

            self.io.resize(frame.len());
            self.remaining -= self.io.len();
        }

        self.io.read(buf)
    }
}

impl<W> Write for FrameIo<W> where W: Write {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.io.is_empty() && !self.is_empty() {
            if buf.len() > self.remaining {
                return Err(ErrorKind::InvalidData.into());
            }

            let frame = Frame::new(FrameType::Data, buf.len());

            self.io.resize(Frame::bytes());
            self.io.write(&(frame.kind() as u8).to_le_bytes())?;
            self.io.write(&(frame.len() as u64).to_le_bytes())?;
            self.io.resize(frame.len());

            self.remaining -= self.io.len();
        }

        self.io.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }
}
