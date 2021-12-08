use std::borrow::Borrow;
use std::fmt;

/// A source for lexemes that can be indexed via a pair of `Location`s.
pub trait LexemeSource {
    fn lexeme(&self, start: &Location, end: &Location) -> &Self;
}

impl LexemeSource for str {
    fn lexeme(&self, start: &Location, end: &Location) -> &str {
        &self[start.offset..end.offset]
    }
}

/// The line and column of the start of a lexeme.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
        self.offset += '\n'.len_utf8();
    }

    /// Adds a single column to this `Location`.
    pub fn add_column<T: Borrow<char>>(&mut self, character: T) {
        self.column += 1;
        self.offset += character.borrow().len_utf8();
    }

    /// Adds a single offset, without incrementing the column, to this `Location`.
    pub fn add_offset<T: Borrow<char>>(&mut self, character: T) {
        self.offset += character.borrow().len_utf8();
    }

    /// Returns the a `Location` with an offset of 0, but the same line and column.
    pub fn continuation(&self) -> Location {
        Location {
            line: self.line,
            column: self.column,
            offset: 0,
        }
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
