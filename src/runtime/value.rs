//! Valid values in the Tortuga runtime.

use crate::runtime::environment::FunctionReference;
use crate::runtime::epsilon::Epsilon;
use crate::runtime::{Number, Tolerance};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign,
    Rem, RemAssign, Sub, SubAssign,
};

/// A value that may be created by a literal, or returned from a function.
#[derive(Copy, Clone, Debug)]
pub enum Value {
    Unit,
    Boolean(bool),
    Number(Number),
    Tolerance(Tolerance),
    FunctionReference(FunctionReference),
}

impl<I: Into<Value>> Epsilon<I> for Value {
    type Output = Value;

    fn epsilon(self, rhs: I) -> Self::Output {
        match (self, rhs.into()) {
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
            Value::FunctionReference(reference) => write!(f, "@{}", reference),
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

impl From<FunctionReference> for Value {
    fn from(reference: FunctionReference) -> Self {
        Value::FunctionReference(reference)
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

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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

impl MulAssign for Value {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
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

impl DivAssign for Value {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
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

impl RemAssign for Value {
    fn rem_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => *a %= b,
            (a, _) => *a = Self::Unit,
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

impl BitXorAssign for Value {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
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

impl BitAndAssign for Value {
    fn bitand_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Value::Boolean(a), Value::Boolean(b)) => *a &= b,
            (v, _) => *v = Self::Unit,
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
            (Value::FunctionReference(a), Value::FunctionReference(b)) => a == b,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_number() {
        let a = Value::from(1);
        let b = Value::from(1);
        let mut c = a;

        c += b;

        assert_eq!(a + b, c);
        assert_eq!(c, 2.into());
    }

    #[test]
    fn add_number_tolerance() {
        let a = Value::from(1);
        let b = Value::from(Number::from(1).epsilon(2));
        let mut c = a;

        c += b;

        assert_eq!(a + b, c);
        assert_eq!(c, Tolerance::new(2, 2).into());
    }

    #[test]
    fn add_tolerance_number() {
        let a = Value::from(Tolerance::new(1, 2));
        let b = Value::from(1);
        let mut c = a;

        c += b;

        assert_eq!(a + b, c);
        assert_eq!(c, Tolerance::new(2, 2).into());
    }

    #[test]
    fn add_tolerance() {
        let a = Value::from(Tolerance::new(1, 2));
        let b = Value::from(Tolerance::new(1, 2));
        let mut c = a;

        c += b;

        assert_eq!(a + b, c);
        assert_eq!(c, Tolerance::new(2, 4).into());
    }

    #[test]
    fn add_other() {
        let a = Value::from(false);
        let b = Value::from(1);

        assert_eq!(a + b, Value::Unit);
    }

    #[test]
    fn sub_number() {
        let a = Value::from(1);
        let b = Value::from(1);
        let mut c = a;

        c -= b;

        assert_eq!(a - b, c);
        assert_eq!(c, 0.into());
    }

    #[test]
    fn sub_number_tolerance() {
        let a = Value::from(1);
        let b = Value::from(Number::from(1).epsilon(2));
        let mut c = a;

        c -= b;

        assert_eq!(a - b, c);
        assert_eq!(c, Tolerance::new(0, 2).into());
    }

    #[test]
    fn sub_tolerance_number() {
        let a = Value::from(Tolerance::new(1, 2));
        let b = Value::from(1);
        let mut c = a;

        c -= b;

        assert_eq!(a - b, c);
        assert_eq!(c, Tolerance::new(0, 2).into());
    }

    #[test]
    fn sub_tolerance() {
        let a = Value::from(Tolerance::new(1, 2));
        let b = Value::from(Tolerance::new(1, 2));
        let mut c = a;

        c -= b;

        assert_eq!(a - b, c);
        assert_eq!(c, Tolerance::new(0, 4).into());
    }

    #[test]
    fn sub_other() {
        let a = Value::from(1);
        let b = Value::from(false);

        assert_eq!(a - b, Value::Unit);
    }

    #[test]
    fn mul_number() {
        let a = Value::from(2);
        let b = Value::from(2);
        let mut c = a;

        c *= b;

        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_number_tolerance() {
        let a = Value::from(2);
        let b = Value::from(Number::from(2).epsilon(2));
        let mut c = a;

        c *= b;

        assert_eq!(a * b, c);
        assert_eq!(c, Tolerance::new(4, 2).into());
    }

    #[test]
    fn mul_tolerance_number() {
        let a = Value::from(Tolerance::new(2, 2));
        let b = Value::from(2);
        let mut c = a;

        c *= b;

        assert_eq!(a * b, c);
        assert_eq!(c, Tolerance::new(4, 2).into());
    }

    #[test]
    fn mul_other() {
        let a = Value::from(1);
        let b = Value::Unit;

        assert_eq!(a * b, Value::Unit);
    }

    #[test]
    fn div_number() {
        let a = Value::from(4);
        let b = Value::from(2);
        let mut c = a;

        c /= b;

        assert_eq!(a / b, c);
    }

    #[test]
    fn div_number_tolerance() {
        let a = Value::from(4);
        let b = Value::from(Number::from(2).epsilon(2));
        let mut c = a;

        c /= b;

        assert_eq!(a / b, c);
        assert_eq!(c, Tolerance::new(2, 2).into());
    }

    #[test]
    fn div_tolerance_number() {
        let a = Value::from(Tolerance::new(4, 2));
        let b = Value::from(2);
        let mut c = a;

        c /= b;

        assert_eq!(a / b, c);
        assert_eq!(c, Tolerance::new(2, 2).into());
    }

    #[test]
    fn div_other() {
        let a = Value::from(true);
        let b = Value::from(1);

        assert_eq!(a / b, Value::Unit);
    }

    #[test]
    fn rem_number() {
        let a = Value::from(5);
        let b = Value::from(3);
        let mut c = a;

        c %= b;

        assert_eq!(a % b, c);
    }

    #[test]
    fn rem_other() {
        let a = Value::from(true);
        let b = Value::from(true);

        assert_eq!(a % b, Value::Unit);
    }

    #[test]
    fn bitxor_number() {
        let a = Value::from(2);
        let b = Value::from(2);
        let mut c = a;

        c ^= b;

        assert_eq!(a ^ b, c);
    }

    #[test]
    fn bitxor_number_tolerance() {
        let a = Value::from(2);
        let b = Value::from(Number::from(2).epsilon(2));
        let mut c = a;

        c ^= b;

        assert_eq!(a ^ b, c);
        assert_eq!(c, Tolerance::new(4, 2).into());
    }

    #[test]
    fn bitxor_tolerance_number() {
        let a = Value::from(Tolerance::new(2, 2));
        let b = Value::from(2);
        let mut c = a;

        c ^= b;

        assert_eq!(a ^ b, c);
        assert_eq!(c, Tolerance::new(4, 2).into());
    }

    #[test]
    fn bitxor_other() {
        let a = Value::from(1);
        let b = Value::from(true);

        assert_eq!(a ^ b, Value::Unit);
    }

    #[test]
    fn bitand_boolean() {
        let a = Value::from(true);
        let b = Value::from(false);
        let mut c = a;

        c &= b;

        assert_eq!(a & b, c);
    }

    #[test]
    fn bitand_other() {
        let a = Value::from(1);
        let b = Value::from(1);

        assert_eq!(a & b, Value::Unit);
    }
}
