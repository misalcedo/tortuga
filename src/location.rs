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
    /// Creates a new instance of a `Location` with the given values.
    #[cfg(test)]
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Location {
            line,
            column,
            offset,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_location() {
        assert_eq!(Location::default(), Location::new(1, 1, 0));
    }

    #[test]
    fn add_offset_to_location_when_ascii() {
        let mut location = Location::default();
        let c = 'a';

        location.add_offset(c);

        assert_eq!(location, Location::new(1, 1, c.len_utf8()));
    }

    #[test]
    fn add_offset_to_location_when_multi_byte() {
        let mut location = Location::default();
        let c = '〞';

        location.add_offset(c);

        assert_eq!(location, Location::new(1, 1, c.len_utf8()));
    }

    #[test]
    fn add_column_to_location_when_ascii() {
        let mut location = Location::default();
        let c = 'a';

        location.add_column(c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn add_column_to_location_when_newline() {
        let mut location = Location::default();
        let c = '\n';

        location.add_column(c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn add_column_to_location_when_multi_byte() {
        let mut location = Location::default();
        let c = '〞';

        location.add_column(c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn next_line_when_first_column() {
        let mut location = Location::default();

        location.next_line();

        assert_eq!(location, Location::new(2, 1, '\n'.len_utf8()));
    }

    #[test]
    fn next_line_when_not_first_column() {
        let mut location = Location::default();
        let c = '〞';

        location.add_column(c);
        location.next_line();
        location.add_column(c);

        assert_eq!(
            location,
            Location::new(2, 2, (2 * c.len_utf8()) + '\n'.len_utf8())
        );
    }

    #[test]
    fn continuation_when_offset_is_zero() {
        let location = Location::default();

        assert_eq!(location.continuation(), location);
    }

    #[test]
    fn continuation_when_not_default() {
        let mut location = Location::default();
        let c = '〞';

        location.add_column(c);
        location.next_line();
        location.add_column(c);

        assert_eq!(location.continuation(), Location::new(2, 2, 0));
    }
}
