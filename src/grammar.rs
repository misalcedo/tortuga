//! The Syntax Tree for the tortuga grammar.

/// A statement in the tortuga grammar.
pub enum Statement {
    Grouping(Grouping),
    Number(Number),
    TextReference(TextReference),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
}

/// Groups a statement to change the order of precedence.
pub struct Grouping {
    statement: Box<Statement>,
}

/// A numeric literal.
pub struct Number {}

/// A text reference.
pub struct TextReference {}

/// An operator that takes 2 arguments.
pub struct BinaryOperation {}

pub enum Operator {
    Multiply,
    Divide,
    Plus,
    Minus,
}

/// An operation that compares 2 arguments relative to each other.
pub struct ComparisonOperation {}

pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    EqualTo,
}
