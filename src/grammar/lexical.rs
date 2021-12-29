//! The lexical grammar rules for Tortuga.

use crate::compiler::Lexeme;

/// The name of a function or constant.
pub struct Identifier(Lexeme);

impl From<Lexeme> for Identifier {
    fn from(lexeme: Lexeme) -> Self {
        Identifier(lexeme)
    }
}

/// A numerical literal.
pub struct Number(Lexeme);

impl From<Lexeme> for Number {
    fn from(lexeme: Lexeme) -> Self {
        Number(lexeme)
    }
}
