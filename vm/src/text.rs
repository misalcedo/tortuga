use crate::Value;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Text(String);

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

macro_rules! impl_from_for_text {
    ($t:ty, $v:ident, $e:expr) => {
        impl From<$t> for Text {
            fn from($v: $t) -> Self {
                Text($e)
            }
        }

        impl From<$t> for Value {
            fn from($v: $t) -> Self {
                Value::Text(Text($e))
            }
        }
    };
}

impl_from_for_text!(&str, text, text.to_string());
impl_from_for_text!(String, text, text);
