//! A program is a series of expressions.
//! Expressions produce values.
//! `Tortuga` has a number of binary operators with different levels of precedence.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use a separate rule for each precedence level to make it explicit.

use crate::grammar::lexical;
use crate::grammar::syntax::{Assignment, List};

/// program → expression+ EOF ;
pub type Expressions = List<Expression>;

/// expression → epsilon | assignment ;
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Expression {
    Epsilon(Box<Epsilon>),
    Assignment(Box<Assignment>),
}

impl From<Epsilon> for Expression {
    fn from(epsilon: Epsilon) -> Self {
        Expression::Epsilon(Box::new(epsilon))
    }
}

impl From<Assignment> for Expression {
    fn from(assignment: Assignment) -> Self {
        Expression::Assignment(Box::new(assignment))
    }
}

/// epsilon → modulo ( "~" modulo )? ;
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

/// modulo → sum ( "%" sum )* ;
pub type Modulo = List<Sum>;

/// sum → product ( ( "+" | "-") product )* ;
pub type Sum = List<Product, AddOrSubtract>;

/// The operator and right-hand side for the `sum` grammar rule.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AddOrSubtract {
    /// +
    Add(Product),
    /// -
    Subtract(Product),
}
/// product → power ( ( "*" | "/" ) power )* ;
pub type Product = List<Power, MultiplyOrDivide>;

/// The operator and right-hand side for the `product` grammar rule.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultiplyOrDivide {
    Multiply(Power),
    Divide(Power),
}

/// power → primary ( "^" primary )* ;
pub type Power = List<Primary>;

/// primary → number | call | grouping ;
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

/// number → "-"? NUMBER ;
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

/// call → IDENTIFIER ( "(" arguments ")" )* ;
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Call {
    identifier: lexical::Identifier,
    arguments: Arguments,
}

impl Call {
    /// Creates a new instance of a `Call` grammar rule.
    pub fn new<I: Into<lexical::Identifier>>(identifier: I, arguments: Arguments) -> Self {
        Call {
            identifier: identifier.into(),
            arguments,
        }
    }
}

/// arguments → expression ( "," expression )* ;
pub type Arguments = List<Expression>;

/// grouping → "(" expression ")" ;
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
