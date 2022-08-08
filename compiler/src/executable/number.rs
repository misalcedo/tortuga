use crate::executable::ParseNumberError;
use crate::grammar;
use std::fmt::{self, Display, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Number(f64);

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number(s.parse::<f64>()?))
    }
}

impl<'a> TryFrom<grammar::Number<'a>> for Number {
    type Error = ParseNumberError;

    fn try_from(number: grammar::Number<'a>) -> Result<Self, Self::Error> {
        Ok(Number(
            number.as_str().parse::<f64>()? * number.sign_number() as f64,
        ))
    }
}
