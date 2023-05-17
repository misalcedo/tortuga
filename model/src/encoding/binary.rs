use crate::encoding::{Encoder, Error};
use bincode;
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};

#[derive(Clone, Copy, Default, Debug)]
pub struct Binary {}

impl<'a, Value> Encoder<Value> for Binary
where
    Value: Serialize + DeserializeOwned,
{
    fn serialize(&self, input: &Value) -> Result<Vec<u8>, Error> {
        bincode::serialize(input).map_err(|_| Error)
    }

    fn serialize_to<Destination>(
        &self,
        destination: &mut Destination,
        input: &Value,
    ) -> Result<(), Error>
    where
        Destination: Write,
    {
        bincode::serialize_into(destination, input).map_err(|_| Error)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<Value, Error> {
        bincode::deserialize(bytes).map_err(|_| Error)
    }

    fn deserialize_from<Source>(&self, source: &mut Source) -> Result<Value, Error>
    where
        Source: Read,
    {
        bincode::deserialize_from(source).map_err(|_| Error)
    }
}
