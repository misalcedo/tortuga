//! A `Location` in source code is both a line and column pair, as well as an offset.
//! The offset is used within the compiler itself.
//! The line and column are used as debugging information.   

use std::fmt;
use std::ops::{Add, AddAssign};

/// The line and column of the start of a lexeme.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Location {
    offset: usize,
    line: usize,
    column: usize,
}

impl Add<&str> for Location {
    type Output = Location;

    fn add(mut self, rhs: &str) -> Self::Output {
        for c in rhs.chars() {
            self.advance(&c);
        }

        self
    }
}

impl AddAssign<&str> for Location {
    fn add_assign(&mut self, rhs: &str) {
        for c in rhs.chars() {
            self.advance(&c);
        }
    }
}

impl From<&str> for Location {
    fn from(s: &str) -> Self {
        Location::default() + s
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl From<Location> for usize {
    fn from(location: Location) -> usize {
        location.offset
    }
}

impl Location {
    /// Creates a new instance of a `Location` with the given values.
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Location {
            line,
            column,
            offset,
        }
    }

    /// The offset of this `Location` into the corresponding input.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Advance this `Location` based on the given character `c`.
    pub fn advance(&mut self, c: &char) {
        match c {
            '\n' => self.next_line(),
            _ => self.add_column(c),
        }
    }

    /// Moves this `Location` to the next line, first column.
    fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.offset += '\n'.len_utf8();
    }

    /// Adds a single column to this `Location`.
    fn add_column(&mut self, c: &char) {
        self.column += 1;
        self.offset += c.len_utf8();
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
    fn advance_location_when_ascii() {
        let mut location = Location::default();
        let c = 'a';

        location.advance(&c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn advance_location_when_multi_byte() {
        let mut location = Location::default();
        let c = '〞';

        location.advance(&c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn next_line_when_first_column() {
        let mut location = Location::default();
        let c = '\n';

        location.advance(&c);

        assert_eq!(location, Location::new(2, 1, c.len_utf8()));
    }

    #[test]
    fn next_line_when_not_first_column() {
        let mut location = Location::default();
        let c = '〞';

        location.advance(&c);
        location.advance(&'\n');
        location.advance(&c);

        assert_eq!(
            location,
            Location::new(2, 2, (2 * c.len_utf8()) + '\n'.len_utf8())
        );
    }

    #[test]
    fn add_string() {
        let text = "abc\n123";
        let location = Location::new(2, 4, 7);

        assert_eq!(Location::default() + text, location);
        assert_eq!(location, text.into());
    }
}
