mod arguments;
mod errors;
mod parse;
mod prompt;
mod run;
mod scan;

pub use errors::CommandLineError;
use prompt::run_prompt;
use run::run;

use std::io::ErrorKind::BrokenPipe;

use crate::arguments::{Arguments, Commands};
use crate::parse::parse_file;
use crate::scan::scan_file;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
        Commands::Parse(command) => parse_file(command.input.to_string().as_str()),
        Commands::Scan(command) => scan_file(command.input.to_string().as_str()),
    }
}
