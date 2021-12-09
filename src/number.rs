//! Parses numeric literals into a syntax tree node.

use std::fmt;

/// Maximum supported radix for numbers.
pub const MAX_RADIX: u32 = 36;

/// The radix for decimal (0-9) numbers.
pub const DECIMAL_RADIX: u32 = 10;

/// Represents an number with both an integer and fractional portion.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Number {
    sign: Option<Sign>,
    integer: u128,
    fraction: Fraction,
}

impl From<Number> for f64 {
    fn from(number: Number) -> Self {
        f64::from(number.sign.unwrap_or_default()) * ((number.integer as f64) + f64::from(number.fraction))
    }
}

impl Number {
    /// Creates a number with the given sign.
    pub fn new(sign: Option<Sign>, integer: u128, fraction: Fraction) -> Self {
        Number {
            sign,
            integer,
            fraction,
        }
    }

    /// Creates a integer number with the given sign.
    #[cfg(test)]
    pub fn new_integer(integer: u128) -> Self {
        Number {
            sign: None,
            integer,
            fraction: Fraction::default(),
        }
    }

    /// Sets the sign of this number.
    pub fn set_sign(&mut self, sign: Sign) {
        self.sign = Some(sign);
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}

/// The sign of a number. Either positive or negative.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sign {
    Positive,
    Negative,
}

impl From<&str> for Sign {
    fn from(sign: &str) -> Self {
        match sign {
            "-" => Self::Negative,
            _ => Self::Positive,
        }
    }
}

impl Default for Sign {
    fn default() -> Self {
        Self::Positive
    }
}

impl From<Sign> for f64 {
    fn from(sign: Sign) -> Self {
        match sign {
            Sign::Negative => -1.0,
            Sign::Positive => 1.0,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Positive => Ok(()),
            Self::Negative => write!(f, "-"),
        }
    }
}

/// Represents a fractional number.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fraction {
    numerator: u128,
    denominator: u128,
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.numerator as f64) / (self.denominator as f64))
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl From<Fraction> for f64 {
    fn from(fraction: Fraction) -> Self {
        (fraction.numerator as f64) / (fraction.denominator as f64)
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

    #[test]
    fn default_is_zero() {
        assert_eq!(
            Number::default(),
            Number::new(None, 0, Fraction::new(0, 1))
        );
    }

    #[test]
    fn set_sign_positive() {
        let mut number = Number::default();

        number.set_sign(Sign::Positive);

        assert_eq!(
            number,
            Number::new(Some(Sign::Positive), 0, Fraction::new(0, 1))
        );
    }

    #[test]
    fn set_sign_negative() {
        let mut number = Number::default();

        number.set_sign(Sign::Negative);

        assert_eq!(
            number,
            Number::new(Some(Sign::Negative), 0, Fraction::new(0, 1))
        );
    }

    #[test]
    fn set_sign_override() {
        let mut number = Number::default();

        number.set_sign(Sign::Negative);
        number.set_sign(Sign::Positive);

        assert_eq!(
            number,
            Number::new(Some(Sign::Positive), 0, Fraction::new(0, 1))
        );
    }
}
