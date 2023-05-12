use bincode;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Error;

#[derive(Clone, Copy, Default, Debug)]
pub struct Binary {}

impl Binary {
    pub fn serialize<Value>(&self, value: &Value) -> Result<Vec<u8>, Error>
    where
        Value: Serialize,
    {
        bincode::serialize(value).map_err(|_| Error)
    }

    pub fn deserialize<'a, Value>(&self, bytes: &'a [u8]) -> Result<Value, Error>
    where
        Value: Deserialize<'a>,
    {
        bincode::deserialize(bytes).map_err(|_| Error)
    }
}
