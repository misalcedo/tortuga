//! Public interface of the tortuga compiler.

mod errors;
mod scanner;

pub use errors::SyntacticalError;
pub use scanner::{Lexeme, Location, Token};
