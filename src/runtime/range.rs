//! Representation of epsilon ranges within the Tortuga runtime.

use crate::runtime::Number;
use std::fmt;
use std::ops::{Add, BitXor, Div, Mul, Rem, Sub};

/// A range centered around a value.
/// The start and end of the range are inclusive of the center plus and minus a value epsilon.
///
/// Represents a tolerance or margin or error in comparison operations.
/// Adheres to interval arithmetic.
///
/// See <https://en.wikipedia.org/wiki/Interval_arithmetic#Interval_operators>
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct EpsilonRange {
    center: Number,
    epsilon: Number,
}

impl EpsilonRange {
    /// Creates a new instance of an `EpsilonRange` around a given `center`.
    pub fn new<C: Into<Number>, E: Into<Number>>(center: C, epsilon: E) -> Self {
        EpsilonRange {
            center: center.into(),
            epsilon: epsilon.into(),
        }
    }
}

impl fmt::Display for EpsilonRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} \u{C2B1} {}", self.center, self.epsilon)
    }
}

impl<I: Into<Number>> From<I> for EpsilonRange {
    fn from(value: I) -> Self {
        EpsilonRange::new(value, 0.0)
    }
}

impl Add for EpsilonRange {
    type Output = EpsilonRange;

    fn add(self, rhs: Self) -> Self::Output {
        EpsilonRange {
            center: self.center + rhs.center,
            epsilon: self.epsilon + rhs.epsilon,
        }
    }
}

impl Add<Number> for EpsilonRange {
    type Output = EpsilonRange;

    fn add(self, rhs: Number) -> Self::Output {
        EpsilonRange {
            center: self.center + rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Add<EpsilonRange> for Number {
    type Output = EpsilonRange;

    fn add(self, rhs: EpsilonRange) -> Self::Output {
        EpsilonRange {
            center: self + rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl Sub for EpsilonRange {
    type Output = EpsilonRange;

    fn sub(self, rhs: Self) -> Self::Output {
        EpsilonRange {
            center: self.center - rhs.center,
            epsilon: self.epsilon + rhs.epsilon,
        }
    }
}

impl Sub<Number> for EpsilonRange {
    type Output = EpsilonRange;

    fn sub(self, rhs: Number) -> Self::Output {
        EpsilonRange {
            center: self.center - rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Sub<EpsilonRange> for Number {
    type Output = EpsilonRange;

    fn sub(self, rhs: EpsilonRange) -> Self::Output {
        EpsilonRange {
            center: self - rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl Mul<Number> for EpsilonRange {
    type Output = EpsilonRange;

    fn mul(self, rhs: Number) -> Self::Output {
        EpsilonRange {
            center: self.center * rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Mul<EpsilonRange> for Number {
    type Output = EpsilonRange;

    fn mul(self, rhs: EpsilonRange) -> Self::Output {
        EpsilonRange {
            center: self * rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl Div<Number> for EpsilonRange {
    type Output = EpsilonRange;

    fn div(self, rhs: Number) -> Self::Output {
        EpsilonRange {
            center: self.center / rhs,
            epsilon: self.epsilon,
        }
    }
}

impl Div<EpsilonRange> for Number {
    type Output = EpsilonRange;

    fn div(self, rhs: EpsilonRange) -> Self::Output {
        EpsilonRange {
            center: self / rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}

impl BitXor<Number> for EpsilonRange {
    type Output = EpsilonRange;

    fn bitxor(self, rhs: Number) -> Self::Output {
        EpsilonRange {
            center: self.center ^ rhs,
            epsilon: self.epsilon,
        }
    }
}

impl BitXor<EpsilonRange> for Number {
    type Output = EpsilonRange;

    fn bitxor(self, rhs: EpsilonRange) -> Self::Output {
        EpsilonRange {
            center: self ^ rhs.center,
            epsilon: rhs.epsilon,
        }
    }
}
