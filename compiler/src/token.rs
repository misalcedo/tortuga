//! Lexical [`Token`]s for the Tortuga Programming Language.

use crate::Location;
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter, Write};

/// A lexical token is a pair of a [`Lexeme`] and a [`Kind`].
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    lexeme: Cow<'a, str>,
    start: Location,
    kind: TokenKind,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' ({}) at {}", self.lexeme, self.kind, self.start)
    }
}

impl<'a> Token<'a> {
    /// Creates a new instance of a [`Token`] with the given [`Location`], [`Lexeme`] and [`Kind`].
    pub fn new<S, L, K>(start: S, lexeme: L, kind: K) -> Self
    where
        S: Into<Location>,
        L: Into<Cow<'a, str>>,
        K: Into<TokenKind>,
    {
        Token {
            lexeme: lexeme.into(),
            start: start.into(),
            kind: kind.into(),
        }
    }

    /// The actual text this [`Token`] represents in the input.
    pub fn lexeme(&'a self) -> &'a str {
        &self.lexeme
    }

    /// The start [`Location`] of this [`Token`]'s [`Lexeme`].
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// This [`Token`]'s variant.
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

/// The variants of the [`Token`]s and their associated attributes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Number => f.write_str("NUMBER"),
            TokenKind::Identifier => f.write_str("IDENTIFIER"),
            TokenKind::Plus => f.write_char('+'),
            TokenKind::Minus => f.write_char('-'),
            TokenKind::Star => f.write_char('*'),
            TokenKind::Slash => f.write_char('/'),
            TokenKind::Percent => f.write_char('%'),
            TokenKind::Caret => f.write_char('^'),
            TokenKind::Tilde => f.write_char('~'),
            TokenKind::Equal => f.write_char('='),
            TokenKind::NotEqual => f.write_str("<>"),
            TokenKind::LessThan => f.write_char('<'),
            TokenKind::LessThanOrEqualTo => f.write_str("<="),
            TokenKind::GreaterThan => f.write_char('>'),
            TokenKind::GreaterThanOrEqualTo => f.write_str(">="),
            TokenKind::Comma => f.write_char(','),
            TokenKind::Underscore => f.write_char('_'),
            TokenKind::At => f.write_char('@'),
            TokenKind::Exclamation => f.write_char('!'),
            TokenKind::VerticalPipe => f.write_char('|'),
            TokenKind::LeftParenthesis => f.write_char('('),
            TokenKind::RightParenthesis => f.write_char(')'),
            TokenKind::LeftBrace => f.write_char('{'),
            TokenKind::RightBrace => f.write_char('}'),
            TokenKind::LeftBracket => f.write_char('['),
            TokenKind::RightBracket => f.write_char(']'),
            TokenKind::BackTick => f.write_char('`'),
            TokenKind::Pound => f.write_char('#'),
            TokenKind::Dollar => f.write_char('$'),
            TokenKind::Ampersand => f.write_char('&'),
            TokenKind::BackSlash => f.write_char('\\'),
            TokenKind::Colon => f.write_char(':'),
            TokenKind::Semicolon => f.write_char(';'),
            TokenKind::SingleQuote => f.write_char('\''),
            TokenKind::DoubleQuote => f.write_char('"'),
            TokenKind::Dot => f.write_char('.'),
            TokenKind::Question => f.write_char('?'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Location;

    #[test]
    fn token() {
        let lexeme = "ab";
        let start = "";
        let kind = TokenKind::Number;
        let token = Token::new(start, lexeme, kind);

        assert_eq!(token.start(), &Location::default());
        assert_eq!(token.lexeme(), lexeme);
        assert_eq!(token.kind(), &kind);
    }
}
