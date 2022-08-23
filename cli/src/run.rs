//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stdout, Write};
use std::str::FromStr;
use tortuga::{Executable, NullCourier, VirtualMachine};

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str, arguments: &[String]) -> Result<(), CommandLineError> {
    let executable = Executable::from_str(source)?;
    let mut machine = VirtualMachine::new(executable, NullCourier);
    let mut values = Vec::with_capacity(arguments.len());

    for argument in arguments {
        values.push(argument.parse()?);
    }

    match machine.call(0, values.as_slice())? {
        Some(value) => Ok(writeln!(stdout(), "{}", value)?),
        None => Ok(writeln!(stdout(), "")?),
    }
}
