//! The Abstract Syntax Tree (AST) of the complete grammar definition for Tortuga.

mod assignment;
mod comparison;
mod expression;
mod list;

pub use assignment::*;
pub use comparison::*;
pub use expression::*;
pub use list::List;

/// The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure.
/// The root of the grammar matches an entire `Tortuga` program (or a sequence of comparisons to make the interpreter more useful).
///
/// program → expressions | comparisons EOF ;
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Program {
    /// expressions → expression+ ;
    Expressions(Expressions),
    /// comparisons → expression ( comparator expression )+ ;
    Comparisons(Comparisons),
}

impl From<Expressions> for Program {
    fn from(expressions: Expressions) -> Self {
        Program::Expressions(expressions)
    }
}

impl From<Comparisons> for Program {
    fn from(comparisons: Comparisons) -> Self {
        Program::Comparisons(comparisons)
    }
}
