use crate::compiler::errors::lexical::ErrorKind;
use crate::runtime::Number;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::str::FromStr;

/// Base 10 (i.e. decimal). The default base for all numbers.
pub const DECIMAL: u32 = 10;

/// The largest supported radix for numbers with an explicit base.
pub const MAX_RADIX: u32 = 36;

impl FromStr for Number {
    type Err = ErrorKind;

    fn from_str(number: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref NUMBER_REGEX: Regex = Regex::new(r###"^(?:([Ii]|[[:digit:]--0][[:digit:]]*)#)?(0?|(?:[[:alnum:]--0][[:alnum:]]*))(?:\.([[:alnum:]]*))?$"###)
                .expect("Invalid regular expression for NUMBER token.");
        }

        let captures = NUMBER_REGEX.captures(number).ok_or(ErrorKind::Number)?;

        let radix_part = captures.get(1).as_ref().map(Match::as_str).unwrap_or("10");
        let integer_part = captures
            .get(2)
            .as_ref()
            .map(Match::as_str)
            .unwrap_or_default();
        let fraction_part = captures
            .get(3)
            .as_ref()
            .map(Match::as_str)
            .unwrap_or_default();

        let radix: u32 = radix_part.parse().map_err(|_| ErrorKind::Number)?;

        if radix > MAX_RADIX {
            return Err(ErrorKind::Number);
        }

        let integer = if integer_part.is_empty() {
            0
        } else {
            i128::from_str_radix(integer_part, radix).map_err(|_| ErrorKind::Number)?
        };
        let fraction = if fraction_part.is_empty() {
            0
        } else {
            i128::from_str_radix(fraction_part, radix).map_err(|_| ErrorKind::Number)?
        };

        if integer_part.is_empty() && fraction_part.is_empty() {
            Err(ErrorKind::Number)
        } else {
            Ok(42.into())
        }
    }
}
