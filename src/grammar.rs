//! The Syntax Tree for the tortuga grammar.

use crate::number::Number;
use std::fmt;

/// An expression in the tortuga grammar.
#[derive(Clone, Debug)]
pub enum Expression {
    Grouping(Grouping),
    Number(Number),
    TextReference(TextReference),
    Locale(Locale),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Grouping(grouping) => write!(f, "{}", grouping),
            Self::Number(number) => write!(f, "{}", number),
            Self::TextReference(reference) => write!(f, "{}", reference),
            Self::Locale(locale) => write!(f, "{}", locale),
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

/// A language and country pair used to determine which text reference translation to use.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Locale {
    language: Language,
    country: Country,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.language, self.country)
    }
}

/// A 2-letter language code used to denote which translation of a text reference to use.
///
/// See <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Language {
    English,
    Spanish,
    Japanese,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A 2-letter country code used to denote which tranlation of a text reference to use.
///
/// See <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Country {
    UnitedStates,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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

impl fmt::Display for ComparisonOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.comparator, self.left, self.right)
    }
}

/// An operator to compare two items to each other.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    EqualTo,
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LessThan => write!(f, "<",),
            Self::GreaterThan => write!(f, ">"),
            Self::EqualTo => write!(f, "="),
        }
    }
}
