//! The lexical grammar rules for Tortuga.

use crate::compiler::Lexeme;
use crate::WithLexeme;

/// The name of a function or constant.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier(String, Lexeme);

impl Identifier {
    /// Creates a new instance of an [`Identifier`].
    pub fn new(source: &str, lexeme: &Lexeme) -> Self {
        Identifier(lexeme.extract_from(source).to_string(), *lexeme)
    }

    /// The [`str`] representation of this [`Identifier`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl WithLexeme for Identifier {
    fn lexeme(&self) -> &Lexeme {
        &self.1
    }
}

/// A numerical literal.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number(String, Lexeme);

impl Number {
    /// Creates a new instance of a [`Number`].
    pub fn new(source: &str, lexeme: &Lexeme) -> Self {
        Number(lexeme.extract_from(source).to_string(), *lexeme)
    }

    /// The [`str`] representation of this [`Number`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl WithLexeme for Number {
    fn lexeme(&self) -> &Lexeme {
        &self.1
    }
}
