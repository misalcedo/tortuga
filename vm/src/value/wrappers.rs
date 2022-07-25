use crate::{Closure, Identifier, Number, Value};

macro_rules! impl_from_to_value {
    ($t:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::$t(value)
            }
        }
    };
}

impl_from_to_value!(Identifier);
impl_from_to_value!(Number);
impl_from_to_value!(Closure);
