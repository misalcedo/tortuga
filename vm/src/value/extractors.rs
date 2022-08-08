use crate::{Closure, Identifier, Number, Text, Value};

macro_rules! impl_try_from_value {
    ($t:ident) => {
        impl TryFrom<Value> for $t {
            type Error = Value;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$t(inner) => Ok(inner),
                    _ => Err(value),
                }
            }
        }
    };
}

impl_try_from_value!(Identifier);
impl_try_from_value!(Number);
impl_try_from_value!(Text);
impl_try_from_value!(Closure);
