use crate::{Closure, Identifier, Number, Text, Value};
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! impl_try_from_value {
    ($n:ident, $t:ty) => {
        impl TryFrom<Value> for $t {
            type Error = Value;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$n(inner) => Ok(inner),
                    _ => Err(value),
                }
            }
        }
    };
}

impl_try_from_value!(Identifier, Identifier);
impl_try_from_value!(Number, Number);
impl_try_from_value!(Text, Text);
impl_try_from_value!(Closure, Closure);
impl_try_from_value!(Reference, Rc<RefCell<Value>>);
