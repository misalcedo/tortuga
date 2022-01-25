//! The Abstract Syntax Tree (AST) of the complete grammar definition for Tortuga.

mod binding;
mod comparison;
mod expression;
mod list;

pub use binding::*;
pub use comparison::*;
pub use expression::*;
pub use list::List;

/// The syntactic grammar of `Tortuga` is used to parse a linear sequence of tokens into a nested syntax tree structure.
/// The root of the grammar matches an entire `Tortuga` program (or a sequence of comparisons to make the interpreter more useful).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    expressions: Expressions,
}

impl From<Expressions> for Program {
    fn from(expressions: Expressions) -> Self {
        Program { expressions }
    }
}

impl Program {
    pub fn expressions(&self) -> &Expressions {
        &self.expressions
    }
}
