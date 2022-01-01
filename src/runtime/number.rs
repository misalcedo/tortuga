//! Representation of numbers within the Tortuga runtime.

use crate::runtime::epsilon::Epsilon;
use crate::runtime::Tolerance;
use std::fmt;
use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub,
    SubAssign,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Number(f64);

impl<I: Into<Number>> Epsilon<I> for Number {
    type Output = Tolerance;

    fn epsilon(self, epsilon: I) -> Self::Output {
        Tolerance::new(self, epsilon.into())
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

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl RemAssign for Number {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl BitXorAssign for Number {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0.powf(rhs.0);
    }
}
