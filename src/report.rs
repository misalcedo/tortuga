//! Reports errors to the user.

use std::error::Error;
use crate::token::Location;

/// Report an error to the user.
pub fn print<E: Error>(location: Location, error: E) {
    eprintln!("An error occurred on {}: {}", location, error);
}