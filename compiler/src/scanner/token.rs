//! Lexical [`Token`]s for the Tortuga Programming Language.

use crate::{Lexeme, Location};
use std::fmt::{self, Display, Formatter, Write};

/// A lexical token is a pair of a [`Lexeme`] and a [`Kind`].
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    lexeme: Lexeme<'a>,
    start: Location,
    kind: Kind,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} token on {}", self.kind, self.lexeme)
    }
}

impl<'a> Token<'a> {
    /// Creates a new instance of a [`Token`] with the given [`Lexeme`] and [`Kind`].
    pub fn new<S: Into<Location>, L: Into<Lexeme<'a>>, K: Into<Kind>>(start: S, lexeme: L, kind: K) -> Self {
        Token {
            lexeme: lexeme.into(),
            start: start.into()
            kind: kind.into(),
        }
    }

    /// The actual text this [`Token`] represents in the input.
    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }

    /// The start [`Location`] of this [`Lexeme`].
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// This [`Token`]'s variant.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// A [`str`] representing this [`Token`] in the input.
    pub fn as_str(&self) -> &str {
        self.lexeme.as_str()
    }
}

/// The variants of the [`Token`]s and their associated attributes.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    // Literals
    Number,
    Identifier,

    // Punctuation
    /// ~
    Tilde,
    /// `
    BackTick,
    /// !
    Exclamation,
    /// @
    At,
    /// #
    Pound,
    /// $
    Dollar,
    /// %
    Percent,
    /// ^
    Caret,
    /// &
    Ampersand,
    /// *
    Star,
    /// (
    LeftParenthesis,
    /// )
    RightParenthesis,
    /// _
    Underscore,
    /// -
    Minus,
    /// +
    Plus,
    /// =
    Equal,
    /// {
    LeftBrace,
    /// [
    LeftBracket,
    /// }
    RightBrace,
    /// ]
    RightBracket,
    /// |
    VerticalPipe,
    /// \
    BackSlash,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// '
    SingleQuote,
    /// "
    DoubleQuote,
    /// <
    LessThan,
    /// ,
    Comma,
    /// >
    GreaterThan,
    /// .
    Dot,
    /// ?
    Question,
    /// /
    Slash,

    // Multi-character
    /// <>
    NotEqual,
    /// <=
    LessThanOrEqualTo,
    /// >=
    GreaterThanOrEqualTo,
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
            Kind::BackTick => f.write_char('`'),
            Kind::Pound => f.write_char('#'),
            Kind::Dollar => f.write_char('$'),
            Kind::Ampersand => f.write_char('&'),
            Kind::BackSlash => f.write_char('\\'),
            Kind::Colon => f.write_char(':'),
            Kind::Semicolon => f.write_char(';'),
            Kind::SingleQuote => f.write_char('\''),
            Kind::DoubleQuote => f.write_char('"'),
            Kind::Dot => f.write_char('.'),
            Kind::Question => f.write_char('?'),
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
