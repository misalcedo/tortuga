use crate::compiler::errors::lexical::ErrorKind;
use crate::runtime::Number;
use lazy_static::lazy_static;
use regex::{Captures, Match, Regex};
use std::str::FromStr;

/// Base 10 (i.e. decimal). The default base for all numbers.
pub const DECIMAL: u32 = 10;

/// The largest supported radix for numbers with an explicit base.
pub const MAX_RADIX: u32 = 36;

const DEFAULT_NUMBER_PART: &str = "0";
const DEFAULT_RADIX: &str = "10";

impl FromStr for Number {
    type Err = ErrorKind;

    fn from_str(number: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref NUMBER_REGEX: Regex = Regex::new(
                r###"(?x)
                    ^
                    (?: ( [[:digit:]--0] [[:digit:]]{0, 1}) \# )? # Optional non-zero radix; at most 2 digits.
                    ( 0? | (?: [[:alnum:]--0] [[:alnum:]]* ) ) # Interger part; optional when the fractional part is present.
                    (?: \. ( [[:alnum:]]* ) )? # Fractional part; optional when the integer part is present.
                    $
                "###
            )
            .expect("Invalid regular expression for NUMBER token.");
        }

        let captures = NUMBER_REGEX.captures(number).ok_or(ErrorKind::Number)?;

        let radix_part = get_match(&captures, 1).unwrap_or(DEFAULT_RADIX);
        let integer_part = get_match(&captures, 2);
        let fraction_part = get_match(&captures, 3);

        let radix: u32 = radix_part.parse().map_err(|_| ErrorKind::Number)?;

        if radix > MAX_RADIX {
            return Err(ErrorKind::Number);
        }

        let integer = u128::from_str_radix(integer_part.unwrap_or(DEFAULT_NUMBER_PART), radix)
            .map_err(|_| ErrorKind::Number)?;

        let numerator_part = fraction_part.unwrap_or(DEFAULT_NUMBER_PART);
        let numerator =
            u128::from_str_radix(numerator_part, radix).map_err(|_| ErrorKind::Number)?;
        let fraction = (numerator as f64) / (radix as f64).powf(numerator_part.len() as f64);

        if integer_part.is_none() && fraction_part.is_none() {
            Err(ErrorKind::Number)
        } else {
            Ok(Number::from((integer as f64) + fraction))
        }
    }
}

fn get_match<'a>(captures: &Captures<'a>, index: usize) -> Option<&'a str> {
    captures
        .get(index)
        .as_ref()
        .map(Match::as_str)
        .filter(|s| !s.is_empty())
}
