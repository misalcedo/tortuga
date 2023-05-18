pub use binary::Binary;
use std::fmt::{Display, Formatter};
use std::io;

mod binary;

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub trait Format<In, Out = In>: Send + Sync {
    fn serialize(&self, input: &In) -> Result<Vec<u8>, Error>;

    fn serialize_to<Destination>(
        &self,
        destination: &mut Destination,
        input: &In,
    ) -> Result<(), Error>
    where
        Destination: io::Write;

    fn deserialize(&self, bytes: &[u8]) -> Result<Out, Error>;

    fn deserialize_from<Source>(&self, source: &mut Source) -> Result<Out, Error>
    where
        Source: io::Read;
}
