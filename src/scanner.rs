//! Scans a source code file for lexical tokens.

use std::iter::Iterator;

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
pub struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
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

/// Scanner for the tortuga language.
pub struct Scanner<'source> {
    code: &'source str,
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Self {
        Scanner { code }
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source> Iterator for Scanner<'source> {
    // We can refer to this type using Self::Item
    type Item = Token<'source>;
    
    // Consumes the next token from the `Scanner`. 
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}