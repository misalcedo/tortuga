use crate::errors::ValidationError;
use std::borrow::Borrow;
use std::fmt;

/// The line and column of the start of a lexeme.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    line: usize,
    column: usize,
    offset: usize,
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
        self.offset += 1;
    }

    /// Adds a single column to this `Location`.
    pub fn add_column<T: Borrow<char>>(&mut self, character: T) {
        self.column += 1;
        self.offset += character.borrow().len_utf8();
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the location equivalent to adding the given character as a column.
    pub fn successor(&self, character: char) -> Location {
        let mut next = *self;
        next.add_column(character);
        next
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug)]
pub struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
    start: Location,
    validations: Vec<ValidationError>,
}

impl<'source> Token<'source> {
    /// Creates a token with no lexical error.
    pub fn new(
        kind: TokenKind,
        lexeme: &'source str,
        start: Location,
        validations: Vec<ValidationError>,
    ) -> Self {
        Token {
            kind,
            lexeme,
            start,
            validations,
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

    /// The list of validation errors for this token.
    pub fn validations(&self) -> &[ValidationError] {
        self.validations.as_slice()
    }

    /// The list of validation errors for this token.
    pub fn take_validations(&mut self) -> Vec<ValidationError> {
        self.validations.drain(..).collect()
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
