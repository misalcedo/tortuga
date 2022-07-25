//! Errors that may occur in the compilation of Tortuga input.

pub mod lexical;
pub mod number;
pub mod syntactical;

pub use lexical::LexicalError;
pub use number::ParseNumberError;
pub use syntactical::SyntacticalError;
