use std::fmt;

/// The line and column of the start and end (exclusive) of a lexeme.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    line: usize,
    start_column: usize,
    end_column: Option<usize>,
}

impl Location {
    /// The lexeme location at the next line.
    pub fn next_line(&self) -> Self {
        Location {
            line: self.line + 1,
            start_column: 1,
            end_column: None,
        }
    }

    /// Binds a lexical token ending at the current location.
    /// This location is not terminated, only its start column is incremented.__rust_force_expr!
    /// Returns the bound token.
    pub fn bind(&mut self) -> Self {
        let mut cloned = self.clone();

        self.start_column += 1;

        cloned.end_column.insert(self.start_column);
        cloned
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Column: {}", self.line, self.start_column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            line: 1,
            start_column: 1,
            end_column: None,
        }
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
    Number,
}
