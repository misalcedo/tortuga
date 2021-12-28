//! Uses a PEG grammar to validate a source file.

use crate::CommandLineError;

use std::io::stdout;
use tortuga::peg::pretty_print;

/// Parses a file with the [PEG](https://pest.rs/book/grammars/peg.html) grammar.
/// Pretty prints the matching grammar rules.
pub fn parse_file(source: &str) -> Result<(), CommandLineError> {
    Ok(pretty_print(source, stdout())?)
}
