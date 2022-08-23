//! Execute a Tortuga source.

use crate::CommandLineError;
use std::io::{stderr, stdout, Write};
use std::str::FromStr;
use tortuga::{Executable, NullCourier, VirtualMachine};

/// Parses the given source as a Tortuga [`Program`] and executes it.
pub fn run(source: &str) -> Result<(), CommandLineError> {
    let executable = Executable::from_str(source)?;
    let mut machine = VirtualMachine::new(executable, NullCourier);

    match machine.call(0, &[]) {
        Ok(Some(value)) => Ok(writeln!(stdout(), "{}", value)?),
        Ok(None) => Ok(writeln!(stdout(), "")?),
        Err(error) => Ok(writeln!(stderr(), "{}", error)?),
    }
}
