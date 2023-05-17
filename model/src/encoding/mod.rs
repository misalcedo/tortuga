pub use binary::Binary;
use std::io;

mod binary;

#[derive(Debug)]
pub struct Error;

pub trait Encoder<In, Out = In> {
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
