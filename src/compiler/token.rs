//! Lexical `Token`s for Tortuga.

use crate::compiler::Lexeme;
use crate::runtime::Number;
use std::convert::{TryFrom, TryInto};

/// A lexical token is a pair of a `Lexeme` and a `Kind`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token {
    lexeme: Lexeme,
    kind: Kind,
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
    pub fn attribute<A: TryFrom<Kind>>(&self) -> Option<A> {
        self.kind.try_into().ok()
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
    DecimalNumber(Number),
    NumberWithBase(Number),
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

    // Delimiters
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}

impl TryFrom<Kind> for Anonymity {
    type Error = ();

    fn try_from(value: Kind) -> Result<Self, Self::Error> {
        match value {
            Kind::Identifier(anonymity) => Ok(anonymity),
            _ => Err(()),
        }
    }
}

impl TryFrom<Kind> for Number {
    type Error = ();

    fn try_from(value: Kind) -> Result<Self, Self::Error> {
        match value {
            Kind::DecimalNumber(number) | Kind::NumberWithBase(number) => Ok(number),
            _ => Err(()),
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
        let kind = Kind::DecimalNumber(number);
        let token = Token::new(lexeme, kind);

        assert_eq!(token.lexeme(), &Lexeme::new(Location::default(), lexeme));
        assert_eq!(token.kind(), &kind);
        assert_eq!(token.attribute::<Number>(), Some(number));
        assert_eq!(token.attribute::<Anonymity>(), None);
    }
}
