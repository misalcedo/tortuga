//! Generates a syntax tree for a file and prints the scanned structure.

use crate::CommandLineError;
use std::io::{stderr, stdout};
use tortuga::{PrettyPrinter, Program};

/// Pretty print the syntax tree for the given source.
pub fn parse_file(source: &str) -> Result<(), CommandLineError> {
    let mut printer = PrettyPrinter::new(stdout(), stderr());

    match source.parse::<Program>() {
        Ok(program) => printer.print_program(&program)?,
        Err(error) => printer.print_syntactical_error(error)?,
    }

    Ok(())
}
