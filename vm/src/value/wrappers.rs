use crate::{Closure, Identifier, Number, Text, Value};

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
impl_from_to_value!(Text);
impl_from_to_value!(Closure);

macro_rules! impl_transitive_from_for_value {
    ($f:ident, $t:ty) => {
        impl From<$t> for Value {
            fn from(v: $t) -> Self {
                Value::$f($f::from(v))
            }
        }
    };
}

impl_transitive_from_for_value!(Text, &str);
impl_transitive_from_for_value!(Text, String);
impl_transitive_from_for_value!(Number, u8);
impl_transitive_from_for_value!(Number, u16);
impl_transitive_from_for_value!(Number, u32);
impl_transitive_from_for_value!(Number, u64);
impl_transitive_from_for_value!(Number, u128);
impl_transitive_from_for_value!(Number, usize);
impl_transitive_from_for_value!(Number, i8);
impl_transitive_from_for_value!(Number, i16);
impl_transitive_from_for_value!(Number, i32);
impl_transitive_from_for_value!(Number, i64);
impl_transitive_from_for_value!(Number, i128);
impl_transitive_from_for_value!(Number, isize);
impl_transitive_from_for_value!(Number, bool);
impl_transitive_from_for_value!(Number, f32);
impl_transitive_from_for_value!(Number, f64);
