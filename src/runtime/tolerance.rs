//! Representation of epsilon ranges within the Tortuga runtime.

use crate::runtime::Number;
use std::fmt;
use std::ops::{Add, BitXor, Div, Mul, Sub};

/// A range centered around a value.
/// The start and end of the range are inclusive of the center plus and minus a value epsilon.
///
/// Represents a tolerance or margin or error in comparison operations.
/// Adheres to interval arithmetic.
///
/// See <https://en.wikipedia.org/wiki/Interval_arithmetic#Interval_operators>
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Tolerance {
    center: Number,
    epsilon: Number,
}

impl Tolerance {
    /// Creates a new instance of an `Tolerance` around a given `center`.
    pub fn new<C: Into<Number>, E: Into<Number>>(center: C, epsilon: E) -> Self {
        Tolerance {
            center: center.into(),
            epsilon: epsilon.into(),
        }
    }

    /// The minimum value of this [`Tolerance].
    pub fn min(&self) -> Number {
        self.center - self.epsilon
    }

    /// The central value of this [`Tolerance].
    pub fn center(&self) -> Number {
        self.center
    }

    /// The maximum value of this [`Tolerance].
    pub fn max(&self) -> Number {
        self.center + self.epsilon
    }

    /// Tests whether the given number is contained in this [`Tolerance].
    pub fn contains(&self, number: &Number) -> bool {
        &self.min() <= number && number <= &self.max()
    }
}

impl fmt::Display for Tolerance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ± {}", self.center, self.epsilon)
    }
}

impl<I: Into<Number>> From<I> for Tolerance {
    fn from(value: I) -> Self {
        Tolerance::new(value, 0.0)
    }
}

impl Add for Tolerance {
    type Output = Tolerance;

    fn add(self, rhs: Self) -> Self::Output {
        Tolerance {
            center: self.center + rhs.center,
            epsilon: self.epsilon + rhs.epsilon,
        }
    }
}

impl Add<Number> for Tolerance {
    type Output = Tolerance;

    fn add(self, rhs: Number) -> Self::Output {
        Tolerance {
            center: self.center + rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Add<Tolerance> for Number {
    type Output = Tolerance;

    fn add(self, rhs: Tolerance) -> Self::Output {
        Tolerance {
            center: self + rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl Sub for Tolerance {
    type Output = Tolerance;

    fn sub(self, rhs: Self) -> Self::Output {
        Tolerance {
            center: self.center - rhs.center,
            epsilon: self.epsilon + rhs.epsilon,
        }
    }
}

impl Sub<Number> for Tolerance {
    type Output = Tolerance;

    fn sub(self, rhs: Number) -> Self::Output {
        Tolerance {
            center: self.center - rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Sub<Tolerance> for Number {
    type Output = Tolerance;

    fn sub(self, rhs: Tolerance) -> Self::Output {
        Tolerance {
            center: self - rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl Mul<Number> for Tolerance {
    type Output = Tolerance;

    fn mul(self, rhs: Number) -> Self::Output {
        Tolerance {
            center: self.center * rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Mul<Tolerance> for Number {
    type Output = Tolerance;

    fn mul(self, rhs: Tolerance) -> Self::Output {
        Tolerance {
            center: self * rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl Div<Number> for Tolerance {
    type Output = Tolerance;

    fn div(self, rhs: Number) -> Self::Output {
        Tolerance {
            center: self.center / rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Div<Tolerance> for Number {
    type Output = Tolerance;

    fn div(self, rhs: Tolerance) -> Self::Output {
        Tolerance {
            center: self / rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl BitXor<Number> for Tolerance {
    type Output = Tolerance;

    fn bitxor(self, rhs: Number) -> Self::Output {
        Tolerance {
            center: self.center ^ rhs,
            epsilon: self.epsilon,
        }
    }
}

impl BitXor<Tolerance> for Number {
    type Output = Tolerance;

    fn bitxor(self, rhs: Tolerance) -> Self::Output {
        Tolerance {
            center: self ^ rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}
