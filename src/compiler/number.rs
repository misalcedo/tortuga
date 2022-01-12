//! Parse a number lexeme into a runtime `Number`.
//! Also, used to validate the lexical structure of a number.
//! Numbers in Tortuga cannot have leading 0s in the radix or integer portion,
//! and cannot have trailing 0s in the fraction portion.
//! Also, the radix for a number cannot be more than 2 digits.

use crate::compiler::errors::ParseNumberError;
use crate::runtime::Number;
use lazy_static::lazy_static;
use regex::{Captures, Match, Regex};
use std::str::FromStr;

pub const MAX_RADIX: u32 = 36;

const DEFAULT_NUMBER_PART: &str = "0";
const DEFAULT_RADIX: &str = "10";

lazy_static! {
    /// A regular expression used to validate and extract a number from a Lexeme.
    pub static ref NUMBER_REGEX: Regex = Regex::new(
        r###"(?x)
            ^
            (?: ( [[:digit:]--0] [[:digit:]]{0, 1}) \# )?
            (?: 
                (?:
                    ( 0 | [[:alnum:]--0] [[:alnum:]]* )
                    (?: \. ( 0? | [[:alnum:]]*? [[:alnum:]--0] ) )?
                )
                |
                (?: ( 0? ) \. ( 0 | [[:alnum:]]*? [[:alnum:]--0] ) )
            )
            $
        "###
    )
    .expect("Invalid regular expression for NUMBER token.");
}

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(number: &str) -> Result<Self, Self::Err> {
        let captures = NUMBER_REGEX
            .captures(number)
            .ok_or_else(|| ParseNumberError::from(number))?;

        let radix_part = get_match(&captures, 1).unwrap_or(DEFAULT_RADIX);
        let integer_part = get_matches(&captures, &[2, 4, 6]);
        let fraction_part = get_matches(&captures, &[3, 5, 7]);

        let radix: u32 = radix_part
            .parse()
            .map_err(|_| ParseNumberError::from(number))?;

        if radix > MAX_RADIX {
            return Err(number.into());
        }

        let integer = u128::from_str_radix(integer_part.unwrap_or(DEFAULT_NUMBER_PART), radix)
            .map_err(|_| ParseNumberError::from(number))?;

        let numerator_part = fraction_part.unwrap_or(DEFAULT_NUMBER_PART);
        let numerator = u128::from_str_radix(numerator_part, radix)
            .map_err(|_| ParseNumberError::from(number))?;
        let fraction = (numerator as f64) / (radix as f64).powf(numerator_part.len() as f64);

        if integer_part.is_none() && fraction_part.is_none() {
            Err(number.into())
        } else {
            Ok(Number::from((integer as f64) + fraction))
        }
    }
}

fn get_matches<'a>(captures: &Captures<'a>, indices: &[usize]) -> Option<&'a str> {
    for &index in indices {
        if captures.get(index).is_some() {
            return get_match(captures, index);
        }
    }

    None
}

fn get_match<'a>(captures: &Captures<'a>, index: usize) -> Option<&'a str> {
    captures
        .get(index)
        .as_ref()
        .map(Match::as_str)
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_number<I: Into<Number>>(number: &str, value: I) {
        assert_eq!(number.parse::<Number>(), Ok(value.into()));
    }

    #[test]
    fn parse_number() {
        validate_number("0", 0);
        validate_number("0.0", 0);
        validate_number(".0", 0);
        validate_number("0.", 0);
        validate_number("2", 2);
        validate_number("4", 4);
        validate_number("21", 21);
        validate_number("100", 100);
        validate_number(".1", 0.1);
        validate_number(".5", 0.5);
        validate_number("1.0", 1.0);
        validate_number("4.5", 4.5);
        validate_number("0.5", 0.5);
        validate_number("10000.5002", 10000.5002);
        validate_number("7.002", 7.002);

        validate_number("2#0", 0);
        validate_number("16#F", 15);
        validate_number("3#21", 7);
        validate_number("2#100", 4);
        validate_number("2#.1", 0.5);
        validate_number("10#.5", 0.5);
        validate_number("12#1.0", 1.0);
        validate_number("20#4.5", 4.25);
        validate_number("30#0.5", 0.16666666666666666);
        validate_number("36#10000.5002", 1679616.1388900797);
        validate_number("32#7.002", 7.00006103515625);
    }

    fn invalidate_number(number: &str) {
        assert_eq!(
            number.parse::<Number>(),
            Err(ParseNumberError::from(number))
        );
    }

    #[test]
    fn parse_invalid_number() {
        invalidate_number(".");
        invalidate_number("20#.");
        invalidate_number("008#1.0");
        invalidate_number("0#1.0");
        invalidate_number("0008");
        invalidate_number(".1000");
        invalidate_number("2#.100");
        invalidate_number("37#1.0");
        invalidate_number("2#4.0");
        invalidate_number("#1.0");
        invalidate_number("#.");
    }
}
