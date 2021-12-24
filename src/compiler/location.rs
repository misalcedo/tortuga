use std::fmt;
use std::ops::Sub;

impl Sub<Location> for Location {
    type Output = usize;

    fn sub(self, rhs: Location) -> Self::Output {
        self.offset - rhs.offset
    }
}

/// The line and column of the start of a lexeme.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    offset: usize,
    line: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}, Column {}", self.line, self.column)
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

    /// Move this `Location` based on the given character `c`.
    pub fn increment(&mut self, c: char) {
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
    fn add_column(&mut self, c: char) {
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
    fn increment_location_when_ascii() {
        let mut location = Location::default();
        let c = 'a';

        location.increment(c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn increment_location_when_multi_byte() {
        let mut location = Location::default();
        let c = '〞';

        location.increment(c);

        assert_eq!(location, Location::new(1, 2, c.len_utf8()));
    }

    #[test]
    fn next_line_when_first_column() {
        let mut location = Location::default();
        let c = '\n';

        location.increment(c);

        assert_eq!(location, Location::new(2, 1, c.len_utf8()));
    }

    #[test]
    fn next_line_when_not_first_column() {
        let mut location = Location::default();
        let c = '〞';

        location.increment(c);
        location.increment('\n');
        location.increment(c);

        assert_eq!(
            location,
            Location::new(2, 2, (2 * c.len_utf8()) + '\n'.len_utf8())
        );
    }

    #[test]
    fn subtract_locations() {
        let mut start = Location::default();
        let mut end = Location::default();

        end.increment('a');
        end.increment('b');
        end.increment('c');

        assert_eq!(end - start, end.offset());
        assert_eq!(end.offset(), 3);
        assert_eq!(start - end, -3);
    }
}
