use std::fs;
use tortuga::TortugaError;
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Options {
    #[clap(short, long, parse(from_occurrences))]
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
    filename: String
}

#[derive(Subcommand)]
enum Commands {
    Prompt(PromptCommand),
    Run(RunCommand),
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Prompt(PromptCommand)
    }
}

fn main() -> Result<(), TortugaError> {
    let options = Options::parse();

    set_verbosity(options.verbose)?;
    run_subcommand(options)
}

fn set_verbosity(occurrences: usize) -> Result<(), TortugaError> {
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

fn run_subcommand(options: Options) -> Result<(), TortugaError> {
    match options.command.unwrap_or_default() {
        Commands::Run(command) => {
            let source = fs::read_to_string(command.filename)?;
            tortuga::run(source.as_str())
        },
        Commands::Prompt(_) => tortuga::run_prompt()
    }
}
