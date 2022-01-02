//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stdout, Write};
use tortuga::{Interpreter, Program};

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str) -> Result<(), CommandLineError> {
    let program: Program = source.parse()?;

    let mut interpreter = Interpreter::default();
    let value = interpreter.run(program);

    write!(stdout(), "{}", value)?;

    Ok(())
}
