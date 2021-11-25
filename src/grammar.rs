//! The Syntax Tree for the tortuga grammar.

use crate::number::Number;
use std::fmt;

/// A list of zero or more expressions in the tortuga grammar.
#[derive(Clone, Debug)]
pub struct Program {
    expressions: Vec<Expression>
}

impl Program {
    /// Creates a new instance of a program.
     pub fn new(expressions: impl IntoIterator<Item=Expression>) -> Self {
        Program {
            expressions: expressions.into_iter().collect()
        }
     }
}

/// An expression in the tortuga grammar.
#[derive(Clone, Debug)]
pub enum Expression {
    Grouping(Grouping),
    Number(Number),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Grouping(grouping) => write!(f, "{}", grouping),
            Self::Number(number) => write!(f, "{}", number),
            Self::BinaryOperation(operation) => write!(f, "{}", operation),
            Self::ComparisonOperation(operation) => write!(f, "{}", operation),
        }
    }
}

impl From<Grouping> for Expression {
    fn from(grouping: Grouping) -> Self {
        Expression::Grouping(grouping)
    }
}

impl From<Number> for Expression {
    fn from(number: Number) -> Self {
        Expression::Number(number)
    }
}

impl From<BinaryOperation> for Expression {
    fn from(operation: BinaryOperation) -> Self {
        Expression::BinaryOperation(operation)
    }
}

impl From<ComparisonOperation> for Expression {
    fn from(operation: ComparisonOperation) -> Self {
        Expression::ComparisonOperation(operation)
    }
}

/// Groups an expression to change the order of precedence.
#[derive(Clone, Debug)]
pub struct Grouping {
    expression: Box<Expression>,
}

impl Grouping {
    pub fn new(expression: Expression) -> Self {
        Grouping {
            expression: Box::new(expression),
        }
    }

    /// The inner expression of this grouping
    pub fn inner(&self) -> &Expression {
        &self.expression
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.expression)
    }
}

/// An operator that takes 2 arguments.
#[derive(Clone, Debug)]
pub struct BinaryOperation {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
}

impl BinaryOperation {
    pub fn new(left: Expression, operator: Operator, right: Expression) -> Self {
        BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn operator(&self) -> Operator {
        self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Exponent,
    Multiply,
    Divide,
    Add,
    Subtract,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Exponent => write!(f, "^"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
        }
    }
}

/// An operation that compares 2 arguments relative to each other.
#[derive(Clone, Debug)]
pub struct ComparisonOperation {
    left: Box<Expression>,
    comparator: ComparisonOperator,
    right: Box<Expression>,
}

impl ComparisonOperation {
    pub fn new(left: Expression, comparator: ComparisonOperator, right: Expression) -> Self {
        ComparisonOperation {
            left: Box::new(left),
            comparator,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn comparator(&self) -> ComparisonOperator {
        self.comparator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl fmt::Display for ComparisonOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.comparator, self.left, self.right)
    }
}

/// An operator to compare two items to each other.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,
    Comparable,
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LessThan => write!(f, "<",),
            Self::LessThanOrEqualTo => write!(f, "<=",),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanOrEqualTo => write!(f, ">="),
            Self::EqualTo => write!(f, "="),
            Self::NotEqualTo => write!(f, "<>"),
            Self::Comparable => write!(f, "<=>"),
        }
    }
}
