//! A program is a series of expressions.
//! Expressions produce values.
//! `Tortuga` has a number of binary operators with different levels of precedence.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use a separate rule for each precedence level to make it explicit.

use crate::grammar::lexical;
use crate::grammar::lexical::Identifier;
use crate::grammar::syntax::{Assignment, List};

pub type Expressions = List<Expression>;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Arithmetic(Epsilon);

impl Arithmetic {
    /// The [`Epsilon`] grammar rule wrapped by this [`Arithmetic`] rule.
    pub fn epsilon(&self) -> &Epsilon {
        &self.0
    }
}

impl From<Epsilon> for Arithmetic {
    fn from(epsilon: Epsilon) -> Self {
        Arithmetic(epsilon)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AddOrSubtract {
    /// +
    Add(Product),
    /// -
    Subtract(Product),
}
pub type Product = List<Power, MultiplyOrDivide>;

/// The operator and right-hand side for the `product` grammar rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MultiplyOrDivide {
    Multiply(Power),
    Divide(Power),
}

pub type Power = List<Call>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Primary {
    Number(Number),
    Identifier(Identifier),
    Grouping(Grouping),
}

impl From<Number> for Primary {
    fn from(number: Number) -> Self {
        Primary::Number(number)
    }
}

impl From<Identifier> for Primary {
    fn from(identifier: Identifier) -> Self {
        Primary::Identifier(identifier)
    }
}

impl From<Grouping> for Primary {
    fn from(grouping: Grouping) -> Self {
        Primary::Grouping(grouping)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number {
    number: lexical::Number,
    negative: bool,
}

impl Number {
    /// Creates a new instance of a `number` grammar rule.
    pub fn new(negative: bool, number: lexical::Number) -> Self {
        Number { number, negative }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call {
    callee: Primary,
    arguments: Vec<Arguments>,
}

impl Call {
    /// Creates a new instance of a `Call` grammar rule.
    pub fn new(callee: Primary, arguments: Vec<Arguments>) -> Self {
        Call {
            callee,
            arguments,
        }
    }

    /// The callee of the function to [`Call`].
    pub fn callee(&self) -> &Primary {
        &self.callee
    }

    /// The [`Arguments`] to invoke this function [`Call`] with.
    pub fn arguments(&self) -> &[Arguments] {
        &self.arguments
    }
}

pub type Arguments = List<Expression>;

#[derive(Clone, Debug, Eq, PartialEq)]
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
