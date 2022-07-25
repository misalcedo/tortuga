use crate::machine::Identifier;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    #[default]
    Number,
    Closure,
    Identifier,
}

impl TryFrom<Value> for Identifier {
    type Error = Value;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Identifier => Ok(Identifier {}),
            _ => Err(value),
        }
    }
}
