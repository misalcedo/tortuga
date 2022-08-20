use crate::compiler::grammar;
use crate::{Number, ParseNumberError};

impl<'a> TryFrom<grammar::Number<'a>> for Number {
    type Error = ParseNumberError;

    fn try_from(number: grammar::Number<'a>) -> Result<Self, Self::Error> {
        Ok(Number::from(
            number.as_str().parse::<f64>()? * number.sign_number() as f64,
        ))
    }
}
