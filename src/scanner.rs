//! Scans a source code file for lexical tokens.

use std::iter::{Iterator, Peekable};
use std::str::Chars;

/// The line and column of the start and end (exclusive) of a lexeme.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    line: usize,
    start_column: usize,
    end_column: usize,
}

impl Location {
    /// Creates a new location with the given line and column.
    pub fn new(line: usize, start_column: usize, end_column: usize) -> Self {
        Location { line, start_column, end_column }
    }

    /// The line of the lexeme location in the file.
    pub fn line(&self) -> usize {
        self.line
    }

    /// The start column of the lexeme location in the file.
    pub fn start_column(&self) -> usize {
        self.start_column
    }

    /// The end column (exclusive) of the lexeme location in the file.
    pub fn end_column(&self) -> usize {
        self.end_column
    }
}

impl Default for Location {
    fn default() -> Self {
        Location { line: 1, start_column: 1, end_column: 1 }
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
    location: Location,
    remaining: Peekable<Chars<'source>>,
    next: Option<Token<'source>>
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Self {
        Scanner { code, location: Location::default(), remaining: code.chars().peekable(), next: None }
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source> Iterator for Scanner<'source> {
    // We can refer to this type using Self::Item
    type Item = Token<'source>;
    
    // Consumes the next token from the `Scanner`. 
    fn next(&mut self) -> Option<Self::Item> {
        match self.remaining.peek() {
            None => None,
            Some(_) => None
        }
    }
}