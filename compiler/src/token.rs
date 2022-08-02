//! Lexical [`Token`]s for the Tortuga Programming Language.

use crate::Location;
use std::fmt::{self, Display, Formatter, Write};

/// A lexical token is a pair of a [`Lexeme`] and a [`Kind`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token<'a> {
    lexeme: &'a str,
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
    pub fn new<S, K>(start: S, lexeme: &'a str, kind: K) -> Self
    where
        S: Into<Location>,
        K: Into<TokenKind>,
    {
        Token {
            lexeme,
            start: start.into(),
            kind: kind.into(),
        }
    }

    /// The actual text this [`Token`] represents in the input.
    pub fn lexeme(&self) -> &'a str {
        &self.lexeme
    }

    /// The start [`Location`] of this [`Token`]'s [`Lexeme`].
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// The end [`Location`] of this [`Token`]'s [`Lexeme`].
    pub fn end(&self) -> Location {
        self.start + self.lexeme
    }

    /// This [`Token`]'s variant.
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

/// The variants of the [`Token`]s and their associated attributes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum TokenKind {
    // Literals
    Number,
    Identifier,
    Uri,

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
            TokenKind::Uri => f.write_str("URI"),
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
        assert_eq!(token.end(), Location::new(1, 3, 2));
        assert_eq!(token.lexeme(), lexeme);
        assert_eq!(token.kind(), &kind);
    }
}
