use crate::grammar;
use std::fmt::{self, Display, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;
use tortuga_executable::{Number, ParseNumberError};

impl<'a> TryFrom<grammar::Number<'a>> for Number {
    type Error = ParseNumberError;

    fn try_from(number: grammar::Number<'a>) -> Result<Self, Self::Error> {
        Ok(Number(
            number.as_str().parse::<f64>()? * number.sign_number() as f64,
        ))
    }
}
