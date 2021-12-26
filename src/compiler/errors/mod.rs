//! Errors that may occur in the compilation of Tortuga input.

pub mod lexical;
pub mod syntactical;

pub use lexical::LexicalError;
pub use syntactical::SyntacticalError;
