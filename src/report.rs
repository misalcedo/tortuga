//! Reports errors to the user.

use crate::token::Location;
use std::error::Error;

/// Report an error to the user.
pub fn print<E: Error>(location: Location, error: E) {
    eprintln!("An error occurred on {}: {}", location, error);
}
