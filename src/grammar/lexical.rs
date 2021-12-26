//! The lexical grammar rules for Tortuga.

pub enum Sign {
    Positive,
    Negative,
}

impl Default for Sign {
    fn default() -> Self {
        Sign::Positive
    }
}

pub enum Sum {
    Add,
    Subtract,
}

pub enum Product {
    Multiply,
    Divide,
}

pub struct Modulo;

pub struct Power;

pub enum Inequality {
    NotEqual,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    LessThan,
    GreaterThan,
}

pub struct Equality;

pub struct Integer<'a>(&'a str);

pub struct Fraction<'a>(&'a str);

pub struct Real<'a>(Integer<'a>, Fraction<'a>);

pub enum Number<'a> {
    Integer(Integer<'a>),
    Fraction(Fraction<'a>),
    Real(Real<'a>),
}

pub struct Decimal<'a> {
    number: Number<'a>,
}

pub enum Radix<'a> {
    Imaginary,
    Real(Integer<'a>),
}

// Latin for "any root", including `i` for imaginary).
pub struct QuisRadix<'a> {
    radix: Radix<'a>,
    sign: Sign,
    number: Number<'a>,
}

pub struct Name<'a>(&'a str);

pub enum Identifier<'a> {
    Anonymous,
    Name(Name<'a>),
}
