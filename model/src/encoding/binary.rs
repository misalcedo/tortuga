use crate::encoding::Error;
use bincode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug)]
pub struct Binary {}

impl Binary {
    pub fn serialize<Value>(&self, value: &Value) -> Result<Vec<u8>, Error>
    where
        Value: Serialize,
    {
        bincode::serialize(value).map_err(|_| Error)
    }

    pub fn serialized_size<Value>(&self, value: &Value) -> Result<usize, Error>
    where
        Value: Serialize,
    {
        bincode::serialized_size(value)
            .map(|n| n as usize)
            .map_err(|_| Error)
    }

    pub fn deserialize<'a, Value>(&self, bytes: &'a [u8]) -> Result<Value, Error>
    where
        Value: Deserialize<'a>,
    {
        bincode::deserialize(bytes).map_err(|_| Error)
    }
}
