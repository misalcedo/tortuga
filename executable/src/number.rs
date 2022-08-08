use crate::ParseNumberError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Number {
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number(s.parse::<f64>()?))
    }
}

impl From<f64> for Number {
    fn from(float: f64) -> Self {
        Number(float)
    }
}

impl From<f32> for Number {
    fn from(float: f32) -> Self {
        Number(float as f64)
    }
}

impl From<i8> for Number {
    fn from(float: i8) -> Self {
        Number(float as f64)
    }
}

impl From<i16> for Number {
    fn from(float: i16) -> Self {
        Number(float as f64)
    }
}

impl From<i32> for Number {
    fn from(float: i32) -> Self {
        Number(float as f64)
    }
}

impl From<i64> for Number {
    fn from(float: i64) -> Self {
        Number(float as f64)
    }
}

impl From<i128> for Number {
    fn from(float: i128) -> Self {
        Number(float as f64)
    }
}

impl From<u8> for Number {
    fn from(float: u8) -> Self {
        Number(float as f64)
    }
}

impl From<u16> for Number {
    fn from(float: u16) -> Self {
        Number(float as f64)
    }
}

impl From<u32> for Number {
    fn from(float: u32) -> Self {
        Number(float as f64)
    }
}

impl From<u64> for Number {
    fn from(float: u64) -> Self {
        Number(float as f64)
    }
}

impl From<u128> for Number {
    fn from(float: u128) -> Self {
        Number(float as f64)
    }
}

impl From<isize> for Number {
    fn from(float: isize) -> Self {
        Number(float as f64)
    }
}

impl From<usize> for Number {
    fn from(float: usize) -> Self {
        Number(float as f64)
    }
}

impl From<bool> for Number {
    fn from(non_zero: bool) -> Self {
        Number(if non_zero { 1.0 } else { 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_numbers() {
        let expected = Number(10.0);

        assert_eq!(expected, Number::from(10u8));
        assert_eq!(expected, Number::from(10u16));
        assert_eq!(expected, Number::from(10u32));
        assert_eq!(expected, Number::from(10u64));
        assert_eq!(expected, Number::from(10u128));
        assert_eq!(expected, Number::from(10usize));
    }

    #[test]
    fn signed_numbers() {
        let expected = Number(-10.0);

        assert_eq!(expected, Number::from(-10i8));
        assert_eq!(expected, Number::from(-10i16));
        assert_eq!(expected, Number::from(-10i32));
        assert_eq!(expected, Number::from(-10i64));
        assert_eq!(expected, Number::from(-10i128));
        assert_eq!(expected, Number::from(-10isize));
    }

    #[test]
    fn numbers() {
        let expected = Number(-10.0);

        assert_eq!(expected, Number::from(-10f64));
        assert_eq!(expected, Number::from(-10f32));
    }

    #[test]
    fn boolean() {
        assert_eq!(Number(1.0), Number::from(true));
        assert_eq!(Number(0.0), Number::from(false));
    }
}
