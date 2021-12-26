//! Errors that may occur in the compilation of Tortuga input.

mod lexical;
mod syntactical;

pub use lexical::LexicalError;
pub use syntactical::SyntacticalError;
