//! Numbers, tuples, and other runtime data types necessary for compiling Tortuga programs.

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Number {
    Natural(NaturalNumber),
    Real(RealNumber),
    Complex(ComplexNumber),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NaturalNumber(i128);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RealNumber {
    integer: NaturalNumber,
    fraction: FractionalNumber,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FractionalNumber {
    numerator: NaturalNumber,
    denominator: NaturalNumber,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ComplexNumber {
    real: RealNumber,
    imaginary: RealNumber,
}

impl From<NaturalNumber> for Number {
    fn from(value: NaturalNumber) -> Self {
        Number::Natural(value)
    }
}

impl From<RealNumber> for Number {
    fn from(value: RealNumber) -> Self {
        Number::Real(value)
    }
}

impl From<ComplexNumber> for Number {
    fn from(value: ComplexNumber) -> Self {
        Number::Complex(value)
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::Natural(NaturalNumber(value))
    }
}
