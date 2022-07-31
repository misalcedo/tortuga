//! Terminals in the Tortuga grammar are numbers, identifiers and URIs.

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
