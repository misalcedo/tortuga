use std::fmt;

/// The line and column of the start and end (exclusive) of a lexeme.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    line: usize,
    start_column: usize,
    end_column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Column: {}", self.line, self.start_column)
    }
}

impl Location {
    /// Creates a new location.
    pub fn new<R: LocationRangeBounds>(line: usize, range: R) -> Self {
        Location {
            line: line,
            start_column: range.start_column(),
            end_column: range.end_column(),
        }
    }
}

/// A half-open range bounds for locations.
pub trait LocationRangeBounds {
    /// The start column (inclusive) of the lexeme location.
    fn start_column(&self) -> usize;

    /// The end column (exclusive) of the lexeme location.
    fn end_column(&self) -> usize;
}

impl LocationRangeBounds for (usize, &str) {
    fn start_column(&self) -> usize {
        self.0
    }

    fn end_column(&self) -> usize {
        self.0 + self.1.len()
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug)]
pub struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
    location: Location,
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, lexeme: &'source str, location: Location) -> Self {
        Token {
            kind,
            lexeme,
            location,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn columns(&self) -> usize {
        self.location.end_column - self.location.start_column
    }
}

/// The kind of a lexical token.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    // Mathematical Symbols
    LeftParenthesis,
    RightParenthesis,
    ForwardSlash,
    Star,
    Percent,
    Equals,
    Plus,
    Minus,
    LessThan,
    GreaterThan,
    Caret,
    Tilde,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Pipe,

    // Single-character tokens
    Comma,
    Period,
    Underscore,
    Colon,
    BackSlash,
    Dollar,
    Ampersand,
    At,
    Pound,
    Exclamation,
    Question,
    SingleQuote,
    DoubleQuote,
    BackQuote,

    // Comparisons
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Logic (keywords)
    And,
    Or,
    ExclusiveOr,
    
    // Literals
    Identifier,
    Number,
}
