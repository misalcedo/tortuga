//! Reports errors to the user.

use std::error::Error;

/// Report an error to the user.
pub fn print<E: Error>(line: usize, column_range: (usize, usize), error: E) {
    eprintln!("An error occurred on line {} between {:?}: {}", line, column_range, error);
}