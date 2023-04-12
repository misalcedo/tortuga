use std::io::{ErrorKind, Read, Write};
use std::marker::PhantomData;

#[link(wasm_import_module = "stream")]
extern "C" {
    pub fn start() -> u64;
    pub fn read(stream: u64, buffer: *mut u8, length: u32) -> i64;
    pub fn write(stream: u64, buffer: *const u8, length: u32) -> i64;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Bidirectional {}

#[derive(Debug, PartialEq, Clone)]
pub enum ReadOnly {}

#[derive(Debug, PartialEq, Clone)]
pub enum WriteOnly {}

pub trait Readable: private::Sealed {}

pub trait Writable: private::Sealed {}

mod private {
    use super::{Bidirectional, ReadOnly, WriteOnly};

    pub trait Sealed {}

    impl Sealed for Bidirectional {}

    impl Sealed for ReadOnly {}

    impl Sealed for WriteOnly {}
}

impl Readable for Bidirectional {}

impl Readable for ReadOnly {}

impl Writable for Bidirectional {}

impl Writable for WriteOnly {}

#[derive(Debug)]
pub struct Stream<Directionality> {
    identifier: u64,
    marker: PhantomData<Directionality>,
}

impl Stream<Bidirectional> {
    pub fn primary() -> Self {
        Stream {
            identifier: 0,
            marker: Default::default(),
        }
    }

    pub fn new() -> Self {
        Stream {
            identifier: unsafe { start() },
            marker: Default::default(),
        }
    }
}

impl<RW> Stream<RW>
where
    RW: Readable + Writable,
{
    pub fn split(self) -> (Stream<ReadOnly>, Stream<WriteOnly>) {
        let read = Stream {
            identifier: self.identifier,
            marker: Default::default(),
        };
        let write = Stream {
            identifier: self.identifier,
            marker: Default::default(),
        };

        (read, write)
    }
}

impl<R> Read for Stream<R>
where
    R: Readable,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = unsafe { read(self.identifier, buf.as_mut_ptr(), buf.len() as u32) };

        if bytes < 0 {
            Err(ErrorKind::UnexpectedEof.into())
        } else {
            Ok(bytes as usize)
        }
    }
}

impl<W> Write for Stream<W>
where
    W: Writable,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes = unsafe { write(self.identifier, buf.as_ptr(), buf.len() as u32) };

        if bytes < 0 {
            Err(ErrorKind::UnexpectedEof.into())
        } else {
            Ok(bytes as usize)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
