//! Lexical `Token`s for Tortuga.

use crate::compiler::Lexeme;
use std::fmt::{self, Display, Formatter, Write};

/// A lexical token is a pair of a `Lexeme` and a `Kind`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token {
    lexeme: Lexeme,
    kind: Kind,
}

impl Token {
    /// Creates a new instance of a `Token` with the given `Lexeme` and attribute.
    pub fn new<L: Into<Lexeme>, K: Into<Kind>>(lexeme: L, kind: K) -> Self {
        Token {
            lexeme: lexeme.into(),
            kind: kind.into(),
        }
    }

    /// The actual text this `Token` represents in the input.
    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }

    /// This `Token`'s variant.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// The variants of the `Token`s and their associated attributes.
/// Rust does not support
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    Number,
    Identifier,

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

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Number => f.write_str("NUMBER"),
            Kind::Identifier => f.write_str("IDENTIFIER"),
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
        let kind = Kind::Number;
        let token = Token::new(lexeme, kind);

        assert_eq!(token.lexeme(), &Lexeme::new(Location::default(), lexeme));
        assert_eq!(token.kind(), &kind);
    }
}
