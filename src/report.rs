//! Reports errors to the user.

use std::error::Error;
use crate::scanner::Location;

/// Report an error to the user.
pub fn print<E: Error>(location: Location, error: E) {
    eprintln!("An error occurred on line {} between {}: {}", location.line(), location.start(), error);
}