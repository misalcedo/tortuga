//! Scans a source code file for lexical tokens.

use std::iter::{Iterator, Peekable};
use std::str::Chars;
use std::fmt;
use crate::errors::LexicalError;

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
            end_column: None
        }
    }

    /// The lexeme location after the current location.
    pub fn next(&self) -> Self {
        match self.end_column {
            None => Location {
                line: self.line,
                start_column: self.start_column + 1,
                end_column: None
            },
            Some(column) => Location {
                line: self.line,
                start_column: column,
                end_column: None
            }
        }
    }

    /// Expands the current location to include the next column.
    pub fn expand_range(&mut self) {
        let column = self.end_column.get_or_insert(self.start_column);
        *column += 1;
    }

    /// Get the string slice at the location.
    pub fn get<'source>(&self, code: &'source str) -> Option<&'source str> {
        match self.end_column {
            None => None,
            Some(_) => {
                code.lines().nth(self.line - 1)
            }
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Column: {}", self.line, self.start_column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Location { line: 1, start_column: 1, end_column: None }
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug)]
pub struct Token<'source> {
    kind: TokenKind,
    code: &'source str,
    location: Location
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, code: &'source str, location: Location) -> Self {
        Token {
            kind, code, location
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
    Number
}

/// Scanner for the tortuga language.
pub struct Scanner<'source> {
    code: &'source str,
    location: Location,
    remaining: Chars<'source>,
}

impl<'source> Scanner<'source> {
    /// Creates a new `Scanner` for the given source code.
    pub fn new(code: &'source str) -> Self {
        Scanner { code, location: Location::default(), remaining: code.chars() }
    }
}

// Implement `Iterator` of `Token`s for `Scanner`.
impl<'source> Iterator for Scanner<'source> {
    // We can refer to this type using Self::Item
    type Item = Result<Token<'source>, LexicalError>;
    
    // Consumes the next token from the `Scanner`. 
    fn next(&mut self) -> Option<Self::Item> {
        match self.remaining.next() {
            None => None,
            Some('+') => {
                let mut location = self.location;
                location.expand_range();
                
                self.location = location.next();
                
                Some(Ok(Token::new(TokenKind::Plus, self.code, location)))
            },
            Some(_) => Some(Err(LexicalError::Unknown(self.location)))
        }
    }
}