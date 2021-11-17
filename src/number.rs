//! Parses numeric literals into a suntax tree node.

use crate::errors::SyntaxError;
use crate::token::Token;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map, opt};
use nom::sequence::{preceded, tuple};

/// Parses a numeric literal into the correspoding grammar rule.
pub fn into_number(token: Token<'_>, is_positive: bool) -> Result<Number, SyntaxError> {
    let (_, (integer, fraction, radix)) =
        all_consuming::<_, _, nom::error::Error<&str>, _>(tuple((
            map(opt(digit1), |i| i.unwrap_or("0")),
            map(preceded(opt(tag(".")), opt(digit1)), |f| f.unwrap_or("0")),
            map(opt(preceded(tag("#"), digit1)), |r| r.unwrap_or("10")),
        )))(token.lexeme())
        .map_err(|_| SyntaxError::InvalidNumber(token.lexeme().to_string(), token.start()))?;

    let radix = radix
        .parse::<u32>()
        .map_err(|e| SyntaxError::InvalidRadix(e, token.lexeme().to_string(), token.start()))?;

    if radix > 36 {
        return Err(SyntaxError::RadixTooLarge(
            radix,
            token.lexeme().to_string(),
            token.start(),
        ));
    }

    if fraction.len() > u32::MAX as usize {
        return Err(SyntaxError::FractionTooLong(
            fraction.len(),
            token.lexeme().to_string(),
            token.start(),
        ));
    }

    let integer = u128::from_str_radix(integer, radix)
        .map_err(|e| SyntaxError::InvalidInteger(e, token.lexeme().to_string(), token.start()))?;
    let numerator = u128::from_str_radix(fraction, radix)
        .map_err(|e| SyntaxError::InvalidFraction(e, token.lexeme().to_string(), token.start()))?;

    Ok(Number::new(
        is_positive.into(),
        integer,
        Fraction::new(numerator, radix.pow(fraction.len() as u32).into()),
    ))
}

/// Represents an number with both an integer and fractional portion.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Number {
    sign: Sign,
    integer: u128,
    fraction: Fraction,
}

impl Default for Number {
    fn default() -> Self {
        Number {
            sign: Sign::default(),
            integer: 0,
            fraction: Fraction::default(),
        }
    }
}

impl Number {
    /// Creates a number with the given sign.
    pub fn new(sign: Sign, integer: u128, fraction: Fraction) -> Self {
        Number {
            sign,
            integer,
            fraction,
        }
    }

    /// Negates the sign of a number.
    pub fn negate(&mut self) {
        self.sign = self.sign.negate();
    }
}

/// The sign of a number. Either positive or negative.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sign {
    Positive,
    Negative,
}

impl From<bool> for Sign {
    fn from(is_positive: bool) -> Self {
        if is_positive {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

impl Default for Sign {
    fn default() -> Self {
        Self::Positive
    }
}

impl Sign {
    /// Negates the sign (i.e., negative to positve and vice versa).
    pub fn negate(&self) -> Self {
        match self {
            Self::Negative => Self::Positive,
            Self::Positive => Self::Negative,
        }
    }
}

/// Represents a fractional number.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fraction {
    numerator: u128,
    denominator: u128,
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl Fraction {
    /// Creates a fraction.
    pub fn new(numerator: u128, denominator: u128) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Location, TokenKind};

    #[test]
    fn positive_with_fraction_binary() {
        let positive = true;
        let token = Token::new(TokenKind::Number, "1.01#2", Location::default());

        let actual = into_number(token, positive).unwrap();

        assert_eq!(actual, Number::new(Sign::Positive, 1, Fraction::new(1, 4)));
    }

    #[test]
    fn negative_with_fraction_decimal() {
        let positive = false;
        let token = Token::new(TokenKind::Number, "5.25", Location::default());

        let actual = into_number(token, positive).unwrap();

        assert_eq!(
            actual,
            Number::new(Sign::Negative, 5, Fraction::new(25, 100))
        );
    }

    #[test]
    fn positive_fraction_only_hex() {
        let positive = true;
        let token = Token::new(TokenKind::Number, ".25#16", Location::default());

        let actual = into_number(token, positive).unwrap();

        assert_eq!(
            actual,
            Number::new(Sign::Positive, 0, Fraction::new(37, 256))
        );
    }

    #[test]
    fn negative_integer_octal() {
        let positive = false;
        let token = Token::new(TokenKind::Number, "7.#8", Location::default());

        let actual = into_number(token, positive).unwrap();

        assert_eq!(actual, Number::new(Sign::Negative, 7, Fraction::new(0, 8)));
    }

    #[test]
    fn default_is_zero() {
        assert_eq!(
            Number::default(),
            Number::new(Sign::Positive, 0, Fraction::new(0, 1))
        );
    }
}
