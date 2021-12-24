mod errors;
mod prompt;
mod validate;

pub use errors::CommandLineError;
use prompt::run_prompt;
use validate::validate_file;

use std::fs;
use std::io::ErrorKind::BrokenPipe;
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Arguments {
    #[clap(short, long, parse(from_occurrences))]
    /// Make the subcommand more talkative.
    verbose: usize,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
/// Run an interactive prompt to interpret source code in a read-evalue-print loop.
struct PromptCommand;

#[derive(Parser)]
/// Compile and run a file.
struct RunCommand {
    filename: String,
}

#[derive(Parser)]
/// Validates a file is well-formed by parsing the contents and printing the syntax tree.
struct ValidateCommand {
    filename: String,
}

#[derive(Parser)]
/// Performs lexical analysis on a file and prints the annotated token sequence.
struct ScanCommand {
    filename: String,
}

#[derive(Subcommand)]
enum Commands {
    Prompt(PromptCommand),
    Run(RunCommand),
    Scan(ScanCommand),
    Validate(ValidateCommand),
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
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
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
        Commands::Validate(command) => {
            let source = fs::read_to_string(command.filename)?;

            validate_file(source.as_str())
        }
        Commands::Scan(command) => {
            let source = fs::read_to_string(command.filename)?;
            let tokens: Vec<tortuga::Token<'_>> = tortuga::Lexer::from(source).collect();

            for token in tokens {
                println!("{}", token);
            }
        }
    }
}
