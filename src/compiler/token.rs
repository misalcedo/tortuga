//! Lexical [`Token`]s for the Tortuga Programming Language.

use crate::compiler::{Lexeme, Location};
use std::fmt::{self, Display, Formatter, Write};

/// A lexical token is a pair of a [`Lexeme`] and a [`Kind`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token<'a> {
    lexeme: Lexeme<'a>,
    kind: Kind,
}

/// A lexical token is a pair of a [`Lexeme`] and a [`Kind`].
#[derive(Clone, Debug, PartialEq)]
pub struct OwnedToken {
    start: Location,
    lexeme: String,
    kind: Kind,
}

impl OwnedToken {
    /// Creates a new instance of a [`Token`] with the given [`Lexeme`] and [`Kind`].
    pub fn new<K: Into<Kind>>(lexeme: &Lexeme<'_>, kind: K) -> Self {
        OwnedToken {
            start: *lexeme.start(),
            lexeme: lexeme.as_str().to_string(),
            kind: kind.into(),
        }
    }

    /// The start [`Location`] of this [`OwnedToken`].
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// This [`OwnedToken`]'s variant.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// A [`str`] representing this [`OwnedToken`] in the input.
    pub fn as_str(&self) -> &str {
        self.lexeme.as_str()
    }
}

impl From<Token<'_>> for OwnedToken {
    fn from(token: Token<'_>) -> Self {
        OwnedToken::new(&token.lexeme, token.kind)
    }
}

impl Display for OwnedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} token on {}", self.kind, self.start)
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} token on {}", self.kind, self.lexeme)
    }
}

impl<'a> Token<'a> {
    /// Creates a new instance of a [`Token`] with the given [`Lexeme`] and [`Kind`].
    pub fn new<L: Into<Lexeme<'a>>, K: Into<Kind>>(lexeme: L, kind: K) -> Self {
        Token {
            lexeme: lexeme.into(),
            kind: kind.into(),
        }
    }

    /// The actual text this [`Token`] represents in the input.
    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }

    /// This [`Token`]'s variant.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// A [`str`] representing this [`Token`] in the input.
    pub fn as_str(&self) -> &'a str {
        self.lexeme.as_str()
    }
}

/// The variants of the [`Token`]s and their associated attributes.
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
    /// @
    At,
    /// !
    Exclamation,
    /// |
    VerticalPipe,

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
            Kind::At => f.write_char('@'),
            Kind::Exclamation => f.write_char('!'),
            Kind::VerticalPipe => f.write_char('|'),
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
        let kind = Kind::Number;
        let token = Token::new(lexeme, kind);

        assert_eq!(token.lexeme(), &Lexeme::new(Location::default(), lexeme));
        assert_eq!(token.kind(), &kind);
    }
}
