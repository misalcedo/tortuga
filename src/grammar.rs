//! The Syntax Tree for the tortuga grammar.

use crate::number::Number;
use std::fmt;

/// An expression in the tortuga grammar.
#[derive(Clone, Debug)]
pub enum Expression {
    Grouping(Grouping),
    Number(Number),
    TextReference(TextReference),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Grouping(grouping) => write!(f, "{}", grouping),
            Self::Number(number) => write!(f, "{}", number),
            Self::TextReference(reference) => write!(f, "{}", reference),
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

impl From<TextReference> for Expression {
    fn from(reference: TextReference) -> Self {
        Expression::TextReference(reference)
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

/// A reference to internationalized text.
/// Text References are opaque types and thus cannot be manipulated in any way.
/// However, the contents of the reference must be valid UTF-8 text.
///
/// In order to support runtime switching of translations, external text references can be
/// mapped to languages, countrys, and locales in a fallible manner (i.e. can fail).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextReference {
    text: String,
}

impl TextReference {
    pub fn new(text: &str) -> Self {
        TextReference {
            text: text.to_string(),
        }
    }
}

impl fmt::Display for TextReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.text)
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
    Multiply,
    Divide,
    Add,
    Subtract,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
