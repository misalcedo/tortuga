//! Generates a syntax tree for a file and prints the scanned structure.

use crate::CommandLineError;
use std::io::{stderr, stdout, Write};
use tortuga::Program;

/// Pretty print the syntax tree for the given source.
pub fn parse_file(source: &str) -> Result<(), CommandLineError> {
    match source.parse::<Program>() {
        Ok(program) => writeln!(stdout(), "{}", program)?,
        Err(error) => writeln!(stderr(), "{}", error)?,
    }

    Ok(())
}
