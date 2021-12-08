use crate::errors::ValidationError;
use crate::location::Location;
use std::fmt;

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    kind: Kind,
    lexeme: &'source str,
    start: Location,
    validations: Vec<ValidationError>,
}

impl<'source> Token<'source> {
    /// Creates a token with no lexical error.
    pub fn new(
        kind: Kind,
        start: Location,
        lexeme: &'source str,
        validations: Vec<ValidationError>,
    ) -> Self {
        Token {
            kind,
            lexeme,
            start,
            validations,
        }
    }

    pub fn kind(&self) -> Kind {
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
pub enum Kind {
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
    Number,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
