//! Generates a syntax tree for a file and prints the scanned structure.

use crate::CommandLineError;
use colored::*;
use std::io::{stderr, stdout, Write};
use tortuga::{Kind, Number, Parser, Program};

/// Pretty print the syntax tree for the given source.
pub fn parse_file(source: &str) -> Result<(), CommandLineError> {
    let mut std_out = stdout();
    let mut std_err = stderr();

    match source.parse::<Program>() {
        Ok(program) => writeln!(std_out, "{:?}", program)?,
        Err(error) => writeln!(std_err, "[{}] {:?}", "ERROR".red().bold(), error)?,
    }

    Ok(())
}
