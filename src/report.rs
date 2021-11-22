//! Reports errors to the user.

use crate::errors::TortugaError;

/// Report an error to the user.
pub fn print(_code: &str, error: TortugaError) {
    eprintln!("{}", error)
}
