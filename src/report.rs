//! Reports errors to the user.

use crate::errors::{TortugaError, LexicalError};

/// Report a lexical error to the user.
pub fn print_lexical(_code: &str, error: LexicalError) {
    eprintln!("{}", error)
}

/// Report an error to the user.
pub fn print(_code: &str, error: TortugaError) {
    eprintln!("{}", error)
}
