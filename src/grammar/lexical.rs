//! The lexical grammar rules for Tortuga.

use crate::compiler::Lexeme;
use crate::WithLexeme;

/// The name of a function or constant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier(Lexeme);

impl From<Lexeme> for Identifier {
    fn from(lexeme: Lexeme) -> Self {
        Identifier(lexeme)
    }
}

impl WithLexeme for Identifier {
    fn lexeme(&self) -> &Lexeme {
        &self.0
    }
}

/// A numerical literal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number(Lexeme);

impl From<Lexeme> for Number {
    fn from(lexeme: Lexeme) -> Self {
        Number(lexeme)
    }
}

impl WithLexeme for Number {
    fn lexeme(&self) -> &Lexeme {
        &self.0
    }
}
