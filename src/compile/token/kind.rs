//! The list of all the different type of lexical tokens.

use std::fmt;

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
