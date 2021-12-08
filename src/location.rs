use std::borrow::Borrow;
use std::fmt;

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

    /// Adds a multiple columns to this `Location`.
    pub fn add_columns(&mut self, lexeme: &str) {
        self.column += lexeme.chars().count();
        self.offset += lexeme.len();
    }

    /// Adds a single column to this `Location`.
    pub fn add_column<T: Borrow<char>>(&mut self, character: T) {
        self.column += 1;
        self.offset += character.borrow().len_utf8();
    }

    /// Returns the subslice of the string slice starting at this `Location`s offset.
    pub fn slice_from<'source>(&self, text: &'source str) -> &'source str {
        &text[self.offset..]
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