mod errors;
mod parse;
mod prompt;
mod scan;

pub use errors::CommandLineError;
use parse::parse_file;
use prompt::run_prompt;

use std::fs;
use std::io::ErrorKind::BrokenPipe;
use std::io::{stdout, Write};
use std::path::PathBuf;
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;

use crate::scan::scan_file;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::InferSubcommands))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Arguments {
    #[clap(short, long, parse(from_occurrences))]
    /// Make the subcommand more talkative.
    verbose: usize,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
/// Run an interactive prompt to interpret source code in a read-evaluate-print loop.
struct PromptCommand;

#[derive(Parser)]
/// Compile and run a file.
struct RunCommand {
    /// The path to the file to compile and run immediately.
    filename: PathBuf,
}

#[derive(Parser)]
/// Parses a file and prints the syntax tree.
struct ParseCommand {
    /// The path to the file to parse into an Abstract Syntax Tree.
    filename: PathBuf,
}

#[derive(Parser)]
/// Performs lexical analysis on a file and prints the annotated token sequence.
struct ScanCommand {
    /// The path to the file to perform lexical analysis on.
    filename: PathBuf,
}

#[derive(Parser)]
/// Print version information.
struct VersionCommand {}

#[derive(Subcommand)]
enum Commands {
    Prompt(PromptCommand),
    Run(RunCommand),
    Scan(ScanCommand),
    Parse(ParseCommand),
    Version(VersionCommand),
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Prompt(PromptCommand)
    }
}

fn main() {
    match execute() {
        Err(CommandLineError::IO(error)) if error.kind() == BrokenPipe => (),
        Err(error) => eprintln!("{}", error),
        Ok(_) => (),
    }
}

fn execute() -> Result<(), CommandLineError> {
    let arguments = Arguments::parse();

    set_verbosity(arguments.verbose)?;
    run_subcommand(arguments)
}

fn set_verbosity(occurrences: usize) -> Result<(), CommandLineError> {
    let level = match occurrences {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    };

    LogTracer::init()?;

    let collector = tracing_subscriber::fmt().with_max_level(level).finish();

    Ok(set_global_default(collector)?)
}

fn run_subcommand(arguments: Arguments) -> Result<(), CommandLineError> {
    match arguments.command.unwrap_or_default() {
        Commands::Prompt(_) => run_prompt(),
        Commands::Run(command) => {
            let source = fs::read_to_string(command.filename)?;

            Ok(tortuga::run(source.as_str()))
        }
        Commands::Parse(command) => {
            let source = fs::read_to_string(command.filename)?;

            parse_file(source.as_str())
        }
        Commands::Scan(command) => {
            let source = fs::read_to_string(command.filename)?;

            scan_file(source.as_str())
        }
        Commands::Version(_) => Ok(writeln!(stdout(), "{}", tortuga::about::VERSION)?),
    }
}
