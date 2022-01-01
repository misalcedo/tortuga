//! The lexical grammar rules for Tortuga.

use crate::compiler::Lexeme;
use crate::WithLexeme;

/// The name of a function or constant.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier(Lexeme, String);

impl Identifier {
    /// Creates a new instance of an [`Identifier`].
    pub fn new(source: &str, lexeme: &Lexeme) -> Self {
        Identifier(*lexeme, lexeme.extract_from(source).to_string())
    }

    /// The [`str`] representation of this [`Identifier`].
    pub fn as_str(&self) -> &str {
        self.1.as_str()
    }
}

impl WithLexeme for Identifier {
    fn lexeme(&self) -> &Lexeme {
        &self.0
    }
}

/// A numerical literal.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number(Lexeme, String);

impl Number {
    /// Creates a new instance of a [`Number`].
    pub fn new(source: &str, lexeme: &Lexeme) -> Self {
        Number(*lexeme, lexeme.extract_from(source).to_string())
    }

    /// The [`str`] representation of this [`Number`].
    pub fn as_str(&self) -> &str {
        self.1.as_str()
    }
}

impl WithLexeme for Number {
    fn lexeme(&self) -> &Lexeme {
        &self.0
    }
}
