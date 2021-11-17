use std::fmt;

/// The line and column of the start of a lexeme.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    line: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}, Column {}", self.line, self.column)
    }
}

impl Location {
    /// Moves this `Location` to the next line, first column.
    pub fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    /// Add the specificied columns to this `Location`.
    pub fn add_columns(&mut self, columns: usize) {
        self.column += columns;
    }
}

impl Default for Location {
    fn default() -> Self {
        Location { line: 1, column: 1 }
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug)]
pub struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
    start: Location,
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, lexeme: &'source str, start: Location) -> Self {
        Token {
            kind,
            lexeme,
            start,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn lexeme(&self) -> &'source str {
        self.lexeme
    }

    pub fn start(&self) -> Location {
        self.start
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

    // Literals
    Identifier,
    Underscore,
    TextReference,
    Number,
    Locale,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
