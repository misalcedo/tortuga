//! The Syntax Tree for the tortuga grammar.

use crate::number::Number;

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

/// A language and country pair used to determine which text reference translation to use.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Locale {
    language: Language,
    country: Country,
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

/// A 2-letter country code used to denote which tranlation of a text reference to use.
///
/// See <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Country {
    UnitedStates,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
}

/// An operation that compares 2 arguments relative to each other.
#[derive(Clone, Debug)]
pub struct ComparisonOperation {
    left: Box<Expression>,
    comparator: ComparisonOperator,
    right: Box<Expression>,
}

/// An operator to compare two items to each other.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    EqualTo,
}
