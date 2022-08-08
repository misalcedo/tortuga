use crate::Value;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other as f64
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<bool> for Number {
    fn eq(&self, other: &bool) -> bool {
        (self.0 != 0.0) == *other
    }
}

macro_rules! impl_from_for_number {
    ($t:ident, $v:ident, $e:expr) => {
        impl From<$t> for Number {
            fn from($v: $t) -> Self {
                Number($e)
            }
        }

        impl From<$t> for Value {
            fn from($v: $t) -> Self {
                Value::Number(Number($e))
            }
        }
    };
}

impl_from_for_number!(f64, number, number);
impl_from_for_number!(i32, number, number as f64);
impl_from_for_number!(bool, number, if number { 1.0 } else { 0.0 });

macro_rules! impl_operator_for_number {
    ($t:ident, $f:ident, $op:tt) => {
        impl $t for Number {
            type Output = Self;

            fn $f(self, other: Self) -> Self {
                Number(self.0 $op other.0)
            }
        }
    };
}

impl_operator_for_number!(Add, add, +);
impl_operator_for_number!(Sub, sub, -);
impl_operator_for_number!(Mul, mul, *);
impl_operator_for_number!(Div, div, /);
impl_operator_for_number!(Rem, rem, %);

macro_rules! impl_assign_operator_for_number {
    ($t:ident, $f:ident, $op:tt) => {
        impl $t for Number {
            fn $f(&mut self, other: Self) {
                self.0 $op other.0;
            }
        }
    };
}

impl_assign_operator_for_number!(AddAssign, add_assign, +=);
impl_assign_operator_for_number!(SubAssign, sub_assign, -=);
impl_assign_operator_for_number!(MulAssign, mul_assign, *=);
impl_assign_operator_for_number!(DivAssign, div_assign, /=);
impl_assign_operator_for_number!(RemAssign, rem_assign, %=);
