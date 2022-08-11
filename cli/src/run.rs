//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stderr, stdout, Write};
use tortuga_compiler::Translation;
use tortuga_vm::{Identifier, Value, VirtualMachine};

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str) -> Result<(), CommandLineError> {
    let executable = Translation::try_from(source)?;
    let mut machine = VirtualMachine::new(executable, ());

    match machine.process(Value::Identifier(Identifier::from(0))) {
        Ok(Some(value)) => Ok(writeln!(stdout(), "{}", value)?),
        Ok(None) => Ok(writeln!(stdout(), "")?),
        Err(error) => Ok(writeln!(stderr(), "{}", error)?),
    }
}
