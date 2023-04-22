use std::io::{self, Read, Write};

pub use decode::Decode;
pub use encode::Encode;
pub use readable::ReadableMessage;
pub use writable::WritableMessage;

mod decode;
mod encode;
mod readable;
mod writable;

pub trait Source {
    fn read_message<M>(self) -> io::Result<M>
    where
        M: ReadableMessage<Self>;
}

impl<R> Source for R
where
    R: Read,
{
    fn read_message<M>(self) -> io::Result<M>
    where
        M: ReadableMessage<Self>,
    {
        M::read_from(self)
    }
}

pub trait Destination {
    fn write_message<M>(&mut self, message: M) -> io::Result<usize>
    where
        M: WritableMessage;
}

impl<W> Destination for W
where
    W: Write,
{
    fn write_message<M>(&mut self, message: M) -> io::Result<usize>
    where
        M: WritableMessage,
    {
        message.write_to(self)
    }
}
