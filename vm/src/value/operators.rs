use crate::Value;

use std::ops::{Add, BitAnd, BitOr, Div, Mul, Not, Rem, Sub};
impl Add for Value {
    type Output = Result<Value, (Value, Value)>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            pair => Err(pair),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, (Value, Value)>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            pair => Err(pair),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, (Value, Value)>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            pair => Err(pair),
        }
    }
}

impl Div for Value {
    type Output = Result<Value, (Value, Value)>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            pair => Err(pair),
        }
    }
}

impl Rem for Value {
    type Output = Result<Value, (Value, Value)>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            pair => Err(pair),
        }
    }
}

impl BitAnd for Value {
    type Output = Result<Value, (Value, Value)>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::from(a != 0 && b != 0)),
            pair => Err(pair),
        }
    }
}

impl BitOr for Value {
    type Output = Result<Value, (Value, Value)>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::from(a != 0 || b != 0)),
            pair => Err(pair),
        }
    }
}

impl Not for Value {
    type Output = Result<Value, Value>;

    fn not(self) -> Self::Output {
        match self {
            Value::Number(a) => Ok(Value::from(a == 0)),
            pair => Err(pair),
        }
    }
}
