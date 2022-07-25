use crate::Value;
use std::ops::{Add, Div, Mul, Rem, Sub};

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
