use crate::errors::ValidationError;
use crate::grammar::{Operator, ComparisonOperator};
use crate::location::Location;
use crate::number::Number;
use std::fmt;

/// A combination of a `Location` and an excerpt from the source code representing the lexeme.
#[derive(Debug, PartialEq)]
pub struct Lexeme<'source> {
    source: &'source str,
    start: Location,
}

impl<'source> Lexeme<'source> {
    /// Creates a new instance of a `Lexeme` with the given `Location` and lexeme.
    pub fn new(source: &'source str, start: Location) -> Self {
        Lexeme {
            source, start
        }
    }

    /// The start `Location` of this `Lexeme`.
    pub fn start(&self) -> &Location {
        &self.start
    }

    /// The source text of this `Lexeme`.
    pub fn source(&self) -> &'source str {
        self.source
    }
}

/// An attachment is used to pass parsed information from the lexical analysis to the parser.
/// This avoids having to duplicate the format of tokens in different places.
#[derive(Debug, PartialEq)]
pub enum Attachment {
    Number(Number),
    Operator(Operator),
    ComparisonOperator(ComparisonOperator),
    Empty(Kind)
}

impl From<&Attachment> for Kind {
    fn from(attachment: &Attachment) -> Self {
        match attachment {
            Attachment::Number(_) => Kind::Number,
            Attachment::Operator(Operator::Add) => Kind::Plus,
            Attachment::Operator(Operator::Subtract) => Kind::Plus,
            Attachment::Operator(Operator::Multiply) => Kind::Minus,
            Attachment::Operator(Operator::Divide) => Kind::ForwardSlash,
            Attachment::Operator(Operator::Exponent) => Kind::Caret,
            Attachment::ComparisonOperator(ComparisonOperator::EqualTo) => Kind::Equals,
            Attachment::ComparisonOperator(ComparisonOperator::LessThan) => Kind::LessThan,
            Attachment::ComparisonOperator(ComparisonOperator::LessThanOrEqualTo) => Kind::LessThan,
            Attachment::ComparisonOperator(ComparisonOperator::GreaterThan) => Kind::GreaterThan,
            Attachment::ComparisonOperator(ComparisonOperator::GreaterThanOrEqualTo) => Kind::GreaterThan,
            Attachment::ComparisonOperator(ComparisonOperator::Comparable) => Kind::LessThan,
            Attachment::Empty(kind) => *kind
        }
    }
}

impl From<Operator> for Attachment {
    fn from(operator: Operator) -> Self {
        Attachment::Operator(operator)
    }
}

impl From<ComparisonOperator> for Attachment {
    fn from(comparator: ComparisonOperator) -> Self {
        Attachment::ComparisonOperator(comparator)
    }
}

impl From<Kind> for Attachment {
    fn from(kind: Kind) -> Self {
        Attachment::Empty(kind)
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug, PartialEq)]
pub enum Token<'source> {
    Valid {
        lexeme: Lexeme<'source>,
        attachment: Attachment
    },
    Invalid {
        kind: Option<Kind>,
        lexeme: Lexeme<'source>,
        validations: Vec<ValidationError>,
    }
}

impl<'source> Token<'source> {
    /// Creates a valid `Token` with no lexical errors.
    pub fn new_valid(
        attachment: Attachment,
        lexeme: Lexeme<'source>,
    ) -> Self {
        Token::Valid {
            attachment, lexeme
        }
    }

    /// Creates an invalid `Token` with one or more lexical errors.
    pub fn new_invalid(
        kind: Option<Kind>,
        lexeme: Lexeme<'source>,
        validations: Vec<ValidationError>,
    ) -> Self {
        Token::Invalid {
            kind,
            lexeme,
            validations,
        }
    }

    pub fn kind(&self) -> Option<Kind> {
        match self {
            Self::Valid { attachment, .. } => Some(attachment.into()),
            Self::Invalid { kind, .. } => *kind
        }
    }

    fn get_lexeme(&self) -> &Lexeme<'source> {
        match self {
            Self::Valid { lexeme, .. } => lexeme,
            Self::Invalid { lexeme, .. } => lexeme
        }
    }

    pub fn lexeme(&self) -> &'source str {
        self.get_lexeme().source()    
    }

    pub fn start(&self) -> &Location {
        let lexeme = match self {
            Self::Valid { lexeme, .. } => lexeme,
            Self::Invalid { lexeme, .. } => lexeme
        };

        lexeme.start()
    }

    /// The list of validation errors for this token.
    pub fn validations(&self) -> &[ValidationError] {
        match self {
            Self::Valid { attachment, .. } => &[],
            Self::Invalid { validations, .. } => validations.as_slice()
        }
    }

    /// The list of validation errors for this token.
    pub fn take_validations(&mut self) -> Vec<ValidationError> {
        match self {
            Self::Valid { attachment, .. } => vec![],
            Self::Invalid { validations, .. } => validations.drain(..).collect()
        }
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
