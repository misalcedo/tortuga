//! A program is a series of expressions.
//! Expressions produce values.
//! `Tortuga` has a number of binary operators with different levels of precedence.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use a separate rule for each precedence level to make it explicit.

use crate::grammar::lexical;
use crate::grammar::syntax::{Assignment, List};

pub type Expressions = List<Expression>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    Arithmetic(Box<Arithmetic>),
    Assignment(Box<Assignment>),
}

impl From<Arithmetic> for Expression {
    fn from(arithmetic: Arithmetic) -> Self {
        Expression::Arithmetic(Box::new(arithmetic))
    }
}

impl From<Assignment> for Expression {
    fn from(assignment: Assignment) -> Self {
        Expression::Assignment(Box::new(assignment))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Arithmetic(Epsilon);

impl From<Epsilon> for Arithmetic {
    fn from(epsilon: Epsilon) -> Self {
        Arithmetic(epsilon)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Epsilon {
    lhs: Modulo,
    rhs: Option<Modulo>,
}

impl Epsilon {
    /// Creates a new instance of the `epsilon` grammar rule.
    pub fn new(lhs: Modulo, rhs: Option<Modulo>) -> Self {
        Epsilon { lhs, rhs }
    }

    /// The left-hand side of this `Epsilon` operation.
    pub fn lhs(&self) -> &Modulo {
        &self.lhs
    }

    /// The right-hand side of this `Epsilon` operation.
    pub fn rhs(&self) -> Option<&Modulo> {
        self.rhs.as_ref()
    }
}

pub type Modulo = List<Sum>;

pub type Sum = List<Product, AddOrSubtract>;

/// The operator and right-hand side for the `sum` grammar rule.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AddOrSubtract {
    /// +
    Add(Product),
    /// -
    Subtract(Product),
}
pub type Product = List<Power, MultiplyOrDivide>;

/// The operator and right-hand side for the `product` grammar rule.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultiplyOrDivide {
    Multiply(Power),
    Divide(Power),
}

pub type Power = List<Primary>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Primary {
    Number(Number),
    Call(Call),
    Grouping(Grouping),
}

impl From<Number> for Primary {
    fn from(number: Number) -> Self {
        Primary::Number(number)
    }
}

impl From<Call> for Primary {
    fn from(call: Call) -> Self {
        Primary::Call(call)
    }
}

impl From<Grouping> for Primary {
    fn from(grouping: Grouping) -> Self {
        Primary::Grouping(grouping)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number {
    negative: bool,
    number: lexical::Number,
}

impl Number {
    /// Creates a new instance of a `number` grammar rule.
    pub fn new<I: Into<lexical::Number>>(negative: bool, number: I) -> Self {
        Number {
            negative,
            number: number.into(),
        }
    }

    /// Tests whether this `Number` represents a negative value.
    pub fn is_negative(&self) -> bool {
        self.negative
    }

    /// Tests whether this `Number` represents a negative value.
    pub fn number(&self) -> &lexical::Number {
        &self.number
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Call {
    identifier: lexical::Identifier,
    arguments: Vec<Arguments>,
}

impl Call {
    /// Creates a new instance of a `Call` grammar rule.
    pub fn new<I: Into<lexical::Identifier>>(identifier: I, arguments: Vec<Arguments>) -> Self {
        Call {
            identifier: identifier.into(),
            arguments,
        }
    }
}

pub type Arguments = List<Expression>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Grouping(Expression);

impl From<Expression> for Grouping {
    fn from(inner: Expression) -> Self {
        Grouping(inner)
    }
}

impl Grouping {
    /// This `Grouping`'s inner `Expression`.
    pub fn inner(&self) -> &Expression {
        &self.0
    }
}
