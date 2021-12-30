//! The Abstract Syntax Tree (AST) of the complete grammar definition for Tortuga.

mod comparison;
mod expression;
mod list;
mod pattern;

pub use comparison::{Comparator, Comparison, Comparisons};
pub use expression::{Expression, Expressions};
pub use list::List;

/// The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure.
/// The root of the grammar matches an entire `Tortuga` program (or a sequence of comparisons to make the interpreter more useful).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Program {
    /// program → expression+ EOF ;
    Expression(Expressions),
    /// program → expression ( comparison expression )+ EOF ;
    Comparison(Comparisons),
}

impl From<Expressions> for Program {
    fn from(expressions: Expressions) -> Self {
        Program::Expression(expressions)
    }
}

impl From<Comparisons> for Program {
    fn from(comparisons: Comparisons) -> Self {
        Program::Comparison(comparisons)
    }
}
