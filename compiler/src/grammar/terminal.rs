//! Terminals in the Tortuga grammar are numbers, identifiers and URIs.

use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Number<'a> {
    negative: bool,
    lexeme: &'a str,
}

impl<'a> Number<'a> {
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
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier<'a> {
    lexeme: &'a str,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uri<'a> {
    lexeme: &'a str,
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
