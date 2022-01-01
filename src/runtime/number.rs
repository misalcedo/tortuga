//! Representation of numbers within the Tortuga runtime.

use crate::runtime::Tolerance;
use std::fmt;
use std::ops::{Add, BitXor, Div, Mul, Rem, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Number {
    /// The ~ operator in Tortuga. Used to create an `Tolerance`.
    pub fn epsilon(&self, epsilon: Number) -> Tolerance {
        Tolerance::new(*self, epsilon)
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
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        if self.0.signum() == rhs.0.signum() {
            Number(self.0 % rhs.0)
        } else {
            Number(self.0 + rhs.0)
        }
    }
}

impl BitXor for Number {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Number(self.0.powf(rhs.0))
    }
}
