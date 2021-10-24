//! Scans a source code file for lexical tokens.

use std::iter::Iterator;

/// Scanner for the tortuga language.
pub struct Scanner {
}

/// The line and column of the start of a lexeme.
#[derive(Debug)]
pub struct Location {
    line: usize,
    start_column: usize
}

impl Location {
    pub fn new(line: usize, start_column: usize) -> Self {
        Location { line, start_column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn start(&self) -> usize {
        self.start_column
    }
}

/// A lexical token.
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    location: Location
}

/// The kind of a lexical token.
#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Pipe,
    Comma,
    Dot,
    Underscore,
    Semicolon,
    COlon,
    ForwardSlash,
    BackSlash,
    Star,
    Dollar,
    Caret,
    Ampersand,
    At,
    Pound,
    Exclamation,
    Percent,
    Equals,
    Plus,
    Minus,
    LessThan,
    GreaterThan,
    Question,
    SingleQuote,
    DoubleQuote,
    BackQuote,
    Tilde,

    // Combination
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Literals
    Identifier,
    Number
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {}
    }

    pub fn scan(&self, code: &str) -> impl Iterator<Item=Token> {
        std::iter::empty::<Token>()
    }
}