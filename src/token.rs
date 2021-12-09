use crate::errors::LexicalError;
use crate::grammar::Operator;
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
    Empty(Kind)
}

impl From<&Attachment> for Kind {
    fn from(attachment: &Attachment) -> Self {
        match attachment {
            Attachment::Number(..) => Kind::Number,
            Attachment::Operator(Operator::Add) => Kind::Plus,
            Attachment::Operator(Operator::Subtract) => Kind::Plus,
            Attachment::Operator(Operator::Multiply) => Kind::Minus,
            Attachment::Operator(Operator::Divide) => Kind::ForwardSlash,
            Attachment::Operator(Operator::Exponent) => Kind::Caret,
            Attachment::Empty(kind) => *kind
        }
    }
}

impl From<Attachment> for Kind {
    fn from(attachment: Attachment) -> Self {
        (&attachment).into()
    }
}

impl From<Operator> for Attachment {
    fn from(operator: Operator) -> Self {
        Attachment::Operator(operator)
    }
}

impl From<Kind> for Attachment {
    fn from(kind: Kind) -> Self {
        Attachment::Empty(kind)
    }
}

/// Trait for a lexical token, which contains a lexeme.
pub trait LexicalToken<'source> {
    /// The `Lexeme` for this `LexicalToken`.
    fn lexeme(&self) -> &Lexeme<'source>;

    /// The excerpt of the source file that represents this `LexicalToken`.
    fn source(&self) -> &'source str {
        self.lexeme().source()    
    }

    /// The start location in the source file of this `LexicalToken`.
    fn start(&self) -> Location {
        *self.lexeme().start()
    }
}

/// A token with no lexical errors.
#[derive(Debug, PartialEq)]
pub struct ValidToken<'source>  {
    lexeme: Lexeme<'source>,
    attachment: Attachment
}

impl<'source> LexicalToken<'source> for ValidToken<'source> {
    fn lexeme(&self) -> &Lexeme<'source> {
        &self.lexeme
    }
}

impl<'source> ValidToken<'source> {
    /// The attached data extracted during lexical analysis.
    pub fn attachment(&self) -> &Attachment {
        &self.attachment
    }

    /// The kind of token that was identified during lexical analysis.
    pub fn kind(&self) -> Kind {
        self.attachment().into()
    }
}

/// A token with one or more lexical errors.
#[derive(Debug, PartialEq)]
pub struct InvalidToken<'source> {
    kind: Option<Kind>,
    lexeme: Lexeme<'source>,
    errors: Vec<LexicalError>,
}

impl<'source> LexicalToken<'source> for InvalidToken<'source> {
    fn lexeme(&self) -> &Lexeme<'source> {
        &self.lexeme
    }
}

impl<'source> InvalidToken<'source> {
    /// The list of lexical errors for this token.
    pub fn errors(&self) -> &[LexicalError] {
        self.errors.as_slice()
    }

    /// The list of lexical errors for this token.
    pub fn take_errors(&mut self) -> Vec<LexicalError> {
        self.errors.drain(..).collect()
    }

    /// The kind of token that was identified during lexical analysis.
    /// If the `Lexer` cannot determine the `Kind` of token, returns `None`.
    /// Otherwise, returns the kind that was being scanned. 
    pub fn kind(&self) -> Option<Kind> {
        self.kind
    }
}

/// A lexical token with a reference to the source.
/// The reference is used when displaying lexemes in errors.
#[derive(Debug, PartialEq)]
pub enum Token<'source> {
    Valid(ValidToken<'source>),
    Invalid(InvalidToken<'source>)
}

impl<'source> Token<'source> {
    /// Creates a new `Token` with potential lexical errors.
    pub fn new(
        attachment: Attachment,
        lexeme: Lexeme<'source>,
        errors: Vec<LexicalError>
    ) -> Self {
        if errors.is_empty() {
            Token::Valid(ValidToken {
                attachment, lexeme
            })
        } else {
            Token::Invalid(InvalidToken {
                kind: Some(attachment.into()),
                lexeme,
                errors,
            })
        }
    }

    /// Creates a valid `Token` with no lexical errors.
    pub fn new_valid(
        attachment: Attachment,
        lexeme: Lexeme<'source>,
    ) -> Self {
        Token::Valid(ValidToken {
            attachment, lexeme
        })
    }

    /// Creates an invalid `Token` with one or more lexical errors.
    pub fn new_invalid(
        kind: Option<Kind>,
        lexeme: Lexeme<'source>,
        errors: Vec<LexicalError>,
    ) -> Self {
        Token::Invalid(InvalidToken {
            kind,
            lexeme,
            errors,
        })
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
