//! Numbers, tuples, and other runtime data types necessary for compiling Tortuga programs.

pub enum Number {
    Natural(NaturalNumber),
    Real(RealNumber),
    Complex(ComplexNumber),
}

pub struct NaturalNumber {}

pub struct RealNumber {
    integer: NaturalNumber,
    fraction: FractionalNumber,
}

pub struct FractionalNumber {}

pub struct ComplexNumber {
    real: RealNumber,
    imaginary: RealNumber,
}
