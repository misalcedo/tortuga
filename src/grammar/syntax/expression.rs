//! A program is a series of expressions.
//! Expressions produce values.
//! `Tortuga` has a number of binary operators with different levels of precedence.
//! Some grammars for languages do not directly encode the precedence relationships and specify that elsewhere.
//! Here, we use a separate rule for each precedence level to make it explicit.

use crate::grammar::lexical;
use crate::grammar::syntax::List;

/// program → expression+ EOF ;
pub struct Expressions(Expression, Vec<Expression>);

/// expression → epsilon | assignment ;
pub enum Expression {
    Epsilon(Box<Epsilon>),
    Assignment,
}

/// epsilon → modulo ( "~" modulo )? ;
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
pub enum AddOrSubtract {
    /// +
    Add(Product),
    /// -
    Subtract(Product),
}
/// product → power ( ( "*" | "/" ) power )* ;
pub type Product = List<Power, MultiplyOrDivide>;

/// The operator and right-hand side for the `product` grammar rule.
pub enum MultiplyOrDivide {
    Multiply(Power),
    Divide(Power),
}

/// power → primary ( "^" primary )* ;
pub type Power = List<Primary>;

/// primary → number | call | grouping ;
pub enum Primary {
    Number,
    Call,
    Grouping,
}

/// number → "-"? NUMBER ;
pub struct Number {
    negative: bool,
    number: lexical::Number,
}

impl Number {
    /// Creates a new instance of a `number` grammar rule.
    pub fn new(negative: bool, number: lexical::Number) -> Self {
        Number { negative, number }
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
pub type Call = List<lexical::Identifier, Arguments>;

/// arguments → expression ( "," expression )* ;
pub type Arguments = List<Expression>;

/// grouping → "(" expression ")" ;
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
