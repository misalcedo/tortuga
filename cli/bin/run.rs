//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stderr, stdout, Write};
use tortuga::Interpreter;

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str) -> Result<(), CommandLineError> {
    match Interpreter::build_then_run(source) {
        Ok(value) => Ok(writeln!(stdout(), "{}", value)?),
        Err(error) => Ok(writeln!(stderr(), "{}", error)?),
    }
}
