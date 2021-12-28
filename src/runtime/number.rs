//! Representation of numbers within the Tortuga runtime.

use crate::runtime::EpsilonRange;
use std::fmt;
use std::ops::{Add, BitXor, Div, Mul, Rem, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Number {
    /// The ~ operator in Tortuga. Used to create an `EpsilonRange`.
    pub fn tilde(&self, epsilon: Number) -> EpsilonRange {
        EpsilonRange::new(*self, epsilon)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<I: Into<f64>> From<I> for Number {
    fn from(value: I) -> Self {
        Number(value.into())
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl Rem for Number {
    type Output = Number;

    fn rem(self, rhs: Self) -> Self::Output {
        if self.0.signum() == rhs.0.signum() {
            Number(self.0 % rhs.0)
        } else {
            Number(self.0 + rhs.0)
        }
    }
}

impl BitXor for Number {
    type Output = Number;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Number(self.0.powf(rhs.0))
    }
}
