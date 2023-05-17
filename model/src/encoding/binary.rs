use crate::encoding::{Encoder, Error};
use bincode;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};

#[derive(Clone, Copy, Default, Debug)]
pub struct Binary {}

impl<'a, In, Out> Encoder<In, Out> for Binary
where
    In: Serialize,
    Out: Deserialize<'a>,
{
    fn serialize(&self, input: &In) -> Result<Vec<u8>, Error> {
        bincode::serialize(value).map_err(|_| Error)
    }

    fn serialize_to<Destination>(
        &self,
        destination: &mut Destination,
        input: &In,
    ) -> Result<(), Error>
    where
        Destination: Write,
    {
        bincode::serialize_into(destination, input).map_err(|_| Error)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Out, Error> {
        bincode::deserialize(bytes).map_err(|_| Error)
    }

    fn deserialize_from<Source>(&self, source: &mut Source) -> Result<Out, Error>
    where
        Source: Read,
    {
        bincode::deserialize_from(source).map_err(|_| Error)
    }
}
