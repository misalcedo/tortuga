//! Errors that may occur in the compilation of Tortuga input.

pub mod number;
pub mod syntactical;

pub use number::ParseNumberError;
pub use syntactical::SyntacticalError;
