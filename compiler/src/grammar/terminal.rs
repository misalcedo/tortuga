//! Terminals in the Tortuga grammar are numbers, identifiers and URIs.

use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Number<'a> {
    negative: bool,
    lexeme: &'a str,
}

impl<'a> Number<'a> {
    pub fn new(negative: bool, lexeme: &'a str) -> Self {
        Number { negative, lexeme }
    }

    pub fn negative(lexeme: &'a str) -> Self {
        Number {
            negative: true,
            lexeme,
        }
    }

    pub fn positive(lexeme: &'a str) -> Self {
        Number {
            negative: false,
            lexeme,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.lexeme
    }

    pub fn sign_number(&self) -> i8 {
        if self.negative {
            -1
        } else {
            1
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier<'a> {
    lexeme: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn as_str(&self) -> &'a str {
        self.lexeme
    }
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(lexeme: &'a str) -> Self {
        Identifier { lexeme }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uri<'a> {
    lexeme: &'a str,
}

impl<'a> Uri<'a> {
    pub fn as_str(&self) -> &'a str {
        self.lexeme
    }
}

impl<'a> From<&'a str> for Uri<'a> {
    fn from(lexeme: &'a str) -> Self {
        Uri { lexeme }
    }
}

impl Display for Number<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?;
        }

        f.write_str(self.lexeme)
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.lexeme)
    }
}

impl Display for Uri<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        f.write_str(self.lexeme)?;
        f.write_char('"')
    }
}
