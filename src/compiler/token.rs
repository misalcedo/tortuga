//! Lexical `Token`s for Tortuga.

use crate::compiler::Lexeme;
use crate::runtime::Number;
use std::fmt;
use std::fmt::{Formatter, Write};

/// A lexical token is a pair of a `Lexeme` and a `Kind`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token {
    lexeme: Lexeme,
    kind: Kind,
}

/// Extra information derived from the text during lexical analysis.
pub trait Attribute {
    /// Get the attribute from the given `Kind` if present, else `None`.
    fn attribute_from(kind: &Kind) -> Option<&Self>;
}

impl Token {
    /// Creates a new instance of a `Token` with the given `Lexeme` and attribute.
    pub fn new<L: Into<Lexeme>>(lexeme: L, kind: Kind) -> Self {
        Token {
            lexeme: lexeme.into(),
            kind,
        }
    }

    /// The actual text the token represents in the input.
    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }

    /// The `Token` variant.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Extra information derived from the text during lexical analysis.
    /// If an attribute `A` can be be extracted from this `Token`'s `Kind`, returns the attribute.
    /// Otherwise, returns `None`.
    pub fn attribute<A: Attribute>(&self) -> Option<&A> {
        A::attribute_from(&self.kind)
    }
}

/// Determines whether an identifier is actually persisted in the environment past its first use.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Anonymity {
    Known,
    Anonymous,
}

/// The variants of the `Token`s and their associated attributes.
/// Rust does not support
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Number(Number),
    Identifier(Anonymity),

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
    /// ,
    Comma,
    /// _
    Underscore,

    // Delimiters
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}

impl Attribute for Anonymity {
    fn attribute_from(value: &Kind) -> Option<&Self> {
        match value {
            Kind::Identifier(anonymity) => Some(anonymity),
            _ => None,
        }
    }
}

impl Attribute for Number {
    fn attribute_from(value: &Kind) -> Option<&Self> {
        match value {
            Kind::Number(number) => Some(number),
            _ => None,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Number(_) => f.write_str("NUMBER"),
            Kind::Identifier(_) => f.write_str("IDENTIFIER"),
            Kind::Plus => f.write_char('+'),
            Kind::Minus => f.write_char('-'),
            Kind::Star => f.write_char('*'),
            Kind::Slash => f.write_char('/'),
            Kind::Percent => f.write_char('%'),
            Kind::Caret => f.write_char('^'),
            Kind::Tilde => f.write_char('~'),
            Kind::Equal => f.write_char('='),
            Kind::NotEqual => f.write_str("<>"),
            Kind::LessThan => f.write_char('<'),
            Kind::LessThanOrEqualTo => f.write_str("<="),
            Kind::GreaterThan => f.write_char('>'),
            Kind::GreaterThanOrEqualTo => f.write_str(">="),
            Kind::Comma => f.write_char(','),
            Kind::Underscore => f.write_char('_'),
            Kind::LeftParenthesis => f.write_char('('),
            Kind::RightParenthesis => f.write_char(')'),
            Kind::LeftBrace => f.write_char('{'),
            Kind::RightBrace => f.write_char('}'),
            Kind::LeftBracket => f.write_char('['),
            Kind::RightBracket => f.write_char(']'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Location;

    #[test]
    fn token() {
        let lexeme = "ab";
        let attribute = 200;
        let number = attribute.into();
        let kind = Kind::Number(number);
        let token = Token::new(lexeme, kind);

        assert_eq!(token.lexeme(), &Lexeme::new(Location::default(), lexeme));
        assert_eq!(token.kind(), &kind);
        assert_eq!(token.attribute::<Number>(), Some(&number));
        assert_eq!(token.attribute::<Anonymity>(), None);
    }
}
