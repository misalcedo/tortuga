//! Public interface of the tortuga compiler.

mod about;

pub use about::*;

mod arguments;
mod errors;
mod prompt;
mod run;

pub use errors::CommandLineError;
use prompt::run_prompt;
use run::run;

use std::io::ErrorKind::BrokenPipe;

use crate::arguments::{Arguments, Commands};

fn main() {
    match execute() {
        Err(CommandLineError::IO(error)) if error.kind() == BrokenPipe => (),
        Err(error) => eprintln!("{error}"),
        Ok(_) => (),
    }
}

fn execute() -> Result<(), CommandLineError> {
    let arguments = Arguments::parse_from_args();

    arguments.verbosity.apply()?;

    run_subcommand(arguments)
}

fn run_subcommand(arguments: Arguments) -> Result<(), CommandLineError> {
    match arguments.command.unwrap_or_default() {
        Commands::Prompt(_) => run_prompt(),
        Commands::Run(command) => run(command.input.to_string().as_str()),
    }
}
