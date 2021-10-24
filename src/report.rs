//! Reports errors to the user.

use std::ops::Range;

/// Report an error to the user.
pub fn report(line: usize, column_range: Range, message: &str) {
    eprintln!("An error occurred on line {} between {}: {}", line, column_range, message);
}