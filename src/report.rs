//! Reports errors to the user.

use std::error::Error;

/// Report an error to the user.
pub fn print<E: Error>(error: E) {
    eprintln!("{}", error);
}
