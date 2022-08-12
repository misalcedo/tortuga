//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stderr, stdout, Write};
use tortuga_compiler::Translation;
use tortuga_vm::VirtualMachine;

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str) -> Result<(), CommandLineError> {
    let executable = Translation::try_from(source)?;
    let mut machine = VirtualMachine::new(executable, ());

    match machine.run() {
        Ok(Some(value)) => Ok(writeln!(stdout(), "{}", value)?),
        Ok(None) => Ok(writeln!(stdout(), "")?),
        Err(error) => Ok(writeln!(stderr(), "{}", error)?),
    }
}
