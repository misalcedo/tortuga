//! Lexical `Token`s for Tortuga.

use crate::compiler::Lexeme;
use std::any::{Any, TypeId};

/// A lexical token is a pair of a `Lexeme` and an generic attribute `T`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Token<A: Any> {
    lexeme: Lexeme,
    attribute: A,
}

impl<L: Into<Lexeme>> From<L> for Token<()> {
    fn from(lexeme: L) -> Self {
        Token {
            lexeme: lexeme.into(),
            attribute: (),
        }
    }
}

impl<A: Any> Token<A> {
    /// Creates a new instance of a `Token` with the given `Lexeme` and attribute.
    pub fn new<L: Into<Lexeme>>(lexeme: L, attribute: A) -> Self {
        Token {
            lexeme: lexeme.into(),
            attribute,
        }
    }

    /// The actual text the token represents in the input.
    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }

    /// Extra information derived from the text during lexical analysis.
    pub fn attribute(&self) -> &A {
        &self.attribute
    }

    /// Tests whether this `Token`s attribute is of a given type.
    pub fn is<T: Any>(&self) -> bool {
        self.attribute.type_id() == TypeId::of::<T>()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TokenKind {
    Number(Token<f64>),
    Identifier(Token<()>),

    // Punctuation
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent,
    /// ^
    Caret,
    /// ~
    Tilde,
    /// =
    Equal,
    /// <>
    NotEqual,
    /// <
    LessThan,
    /// <=
    LessThanOrEqualTo,
    /// >
    GreaterThan,
    /// >=
    GreaterThanOrEqualTo,

    // Delimiters
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Location;

    #[test]
    fn token_matches_type() {
        let token = Token::from("a");

        assert!(token.is::<()>());
        assert!(!token.is::<u16>());
    }

    #[test]
    fn token() {
        let lexeme = "ab";
        let attribute = 24;
        let token = Token::new(lexeme, attribute);

        assert!(token.is::<i32>());
        assert_eq!(token.lexeme(), &Lexeme::new(Location::default(), lexeme));
        assert_eq!(token.attribute(), &attribute);
    }
}
