//! Performs Lexical Analysis on a file and prints the scanned sequence of tokens, their lexemes and attributes.

use crate::CommandLineError;
use std::io::{stderr, stdout};
use tortuga::{PrettyPrinter, Scanner};

/// Pretty print the sequence of tokens for the given source.
pub fn scan_file(source: &str) -> Result<(), CommandLineError> {
    let mut printer = PrettyPrinter::new(stdout(), stderr());

    for (index, result) in Scanner::from(source).enumerate() {
        match result {
            Ok(token) => {
                printer.print(format!("{}) ", index + 1))?;
                printer.print_token(token)?;
            }
            Err(error) => {
                printer.print_err(format!("{}) ", index + 1))?;
                printer.print_lexical_error(error)?;
            }
        }
    }

    Ok(())
}
