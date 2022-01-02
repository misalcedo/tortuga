//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stdout, Write};
use tortuga::Interpreter;

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str) -> Result<(), CommandLineError> {
    let value = Interpreter::build_then_run(source).unwrap_or_default();

    writeln!(stdout(), "{}", value)?;

    Ok(())
}
