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
    Number(Number),
    Identifier(Identifier),
    Grouping(Box<Grouping>),
    Call(Box<Call>),
    Operation(Box<Operation>),
    Assignment(Box<Assignment>),
}

impl From<Number> for Expression {
    fn from(number: Number) -> Self {
        Expression::Number(number)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Expression::Identifier(identifier)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Self {
        Expression::Operation(Box::new(operation))
    }
}

impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Expression::Call(Box::new(call))
    }
}

impl From<Grouping> for Expression {
    fn from(grouping: Grouping) -> Self {
        Expression::Grouping(Box::new(grouping))
    }
}

impl From<Assignment> for Expression {
    fn from(assignment: Assignment) -> Self {
        Expression::Assignment(Box::new(assignment))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Operation {
    pub lhs: Expression,
    pub operator: Operator,
    pub rhs: Expression,
}

impl Operation {
    pub fn new(lhs: Expression, operator: Operator, rhs: Expression) -> Self {
        Operation { lhs, operator, rhs }
    }

    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,
    Tolerance,
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
    callee: Expression,
    arguments: Arguments,
}

impl Call {
    /// Creates a new instance of a `Call` grammar rule.
    pub fn new(callee: Expression, arguments: Arguments) -> Self {
        Call { callee, arguments }
    }

    /// The callee of the function to [`Call`].
    pub fn callee(&self) -> &Expression {
        &self.callee
    }

    /// The [`Arguments`] to invoke this function [`Call`] with.
    pub fn arguments(&self) -> &Arguments {
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
