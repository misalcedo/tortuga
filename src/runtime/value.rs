//! Valid values in the Tortuga runtime.

use crate::runtime::{Number, Tolerance};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, BitAnd, BitXor, Div, Mul, Rem, Sub};

/// A value that may be created by a literal, or returned from a function.
#[derive(Copy, Clone, Debug)]
pub enum Value {
    Unit,
    Boolean(bool),
    Number(Number),
    Tolerance(Tolerance),
}

impl Value {
    pub fn epsilon(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Tolerance(a.epsilon(b)),
            _ => Self::Unit,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit => f.write_str("{}"),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::Number(number) => write!(f, "{}", number),
            Value::Tolerance(tolerance) => write!(f, "{}", tolerance),
        }
    }
}

impl From<Number> for Value {
    fn from(number: Number) -> Self {
        Value::Number(number)
    }
}

impl From<i32> for Value {
    fn from(number: i32) -> Self {
        Value::Number(number.into())
    }
}

impl From<f64> for Value {
    fn from(number: f64) -> Self {
        Value::Number(number.into())
    }
}

impl From<Tolerance> for Value {
    fn from(tolerance: Tolerance) -> Self {
        Value::Tolerance(tolerance)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Unit
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Tolerance(a), Value::Number(b)) => Value::Tolerance(a + b),
            (Value::Number(a), Value::Tolerance(b)) => Value::Tolerance(a + b),
            (Value::Tolerance(a), Value::Tolerance(b)) => Value::Tolerance(a + b),
            _ => Self::Unit,
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Tolerance(a), Value::Number(b)) => Value::Tolerance(a - b),
            (Value::Number(a), Value::Tolerance(b)) => Value::Tolerance(a - b),
            (Value::Tolerance(a), Value::Tolerance(b)) => Value::Tolerance(a - b),
            _ => Self::Unit,
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Tolerance(a), Value::Number(b)) => Value::Tolerance(a * b),
            (Value::Number(a), Value::Tolerance(b)) => Value::Tolerance(a * b),
            _ => Self::Unit,
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            (Value::Tolerance(a), Value::Number(b)) => Value::Tolerance(a / b),
            (Value::Number(a), Value::Tolerance(b)) => Value::Tolerance(a / b),
            _ => Self::Unit,
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Self::Unit,
        }
    }
}

impl BitXor for Value {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a ^ b),
            (Value::Tolerance(a), Value::Number(b)) => Value::Tolerance(a ^ b),
            (Value::Number(a), Value::Tolerance(b)) => Value::Tolerance(a ^ b),
            _ => Self::Unit,
        }
    }
}

impl BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a && b),
            _ => Self::Unit,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Tolerance(a), Value::Tolerance(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Unit, Value::Unit) => true,
            (Value::Number(a), Value::Tolerance(b)) => b.contains(a),
            (Value::Tolerance(a), Value::Number(b)) => a.contains(b),
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (a @ Value::Tolerance(_), b @ Value::Number(_)) => {
                b.partial_cmp(a).map(|x| x.reverse())
            }
            (Value::Number(a), Value::Tolerance(b)) => {
                if a < &b.min() {
                    Some(Ordering::Less)
                } else if a > &b.max() {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Value::Tolerance(a), Value::Tolerance(b)) => {
                if (a.min() <= b.max()) && (a.max() >= b.min()) {
                    a.center().partial_cmp(&b.center())
                } else {
                    a.max().partial_cmp(&b.min())
                }
            }
            _ => None,
        }
    }
}
