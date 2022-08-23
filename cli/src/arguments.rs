use crate::{about, CommandLineError};
use clap::{AppSettings, ArgAction, ArgGroup, Args, Parser, Subcommand};
use std::fs::File;
use std::io::{stdin, Read};
use std::path::PathBuf;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_log::LogTracer;

#[derive(Clone, Debug, Parser, PartialEq)]
#[clap(name = about::PROGRAM, author = about::AUTHORS, version = about::VERSION, about = about::DESCRIPTION)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::InferSubcommands))]
#[clap(global_setting(AppSettings::ArgsNegateSubcommands))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub verbosity: Verbosity,
}

impl Arguments {
    /// Parses an [`Arguments`] instance from the command-line arguments.
    pub fn parse_from_args() -> Self {
        Arguments::parse()
    }
}

/// Set the logging verbosity or level.
#[derive(Args, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Verbosity {
    #[clap(
        short,
        long,
        global(true),
        help_heading("VERBOSITY"),
        conflicts_with_all(&["debug", "trace"]),
        action(ArgAction::Count)
    )]
    /// Make the program more talkative.
    pub verbose: usize,
    #[clap(short, long, global(true), help_heading("VERBOSITY"), conflicts_with_all(&["verbose", "trace"]))]
    /// Print debug messages.
    pub debug: bool,
    #[clap(short, long, global(true), help_heading("VERBOSITY"), conflicts_with_all(&["verbose", "debug"]))]
    /// Print trace messages.
    pub trace: bool,
}

impl Verbosity {
    pub fn apply(&self) -> Result<(), CommandLineError> {
        let mut level = match self.verbose {
            0 => Level::ERROR,
            1 => Level::WARN,
            2 => Level::INFO,
            3 => Level::DEBUG,
            _ => Level::TRACE,
        };

        if self.trace {
            level = Level::TRACE;
        } else if self.debug {
            level = Level::DEBUG;
        }

        LogTracer::init()?;

        let collector = tracing_subscriber::fmt().with_max_level(level).finish();

        Ok(set_global_default(collector)?)
    }
}

#[derive(Clone, Debug, Eq, Parser, PartialEq)]
/// Run an interactive prompt to interpret source code in a read-evaluate-print loop.
pub struct PromptCommand;

#[derive(Clone, Debug, Parser, PartialEq)]
#[clap(group = ArgGroup::new("input").multiple(false))]
/// Compile and run a program from stdin, a file, or inline.
pub struct RunCommand {
    /// The path of a file to use as input.
    #[clap(group = "input")]
    pub path: Option<PathBuf>,
    /// An inline expression to use as input.
    #[clap(short, long, forbid_empty_values(true), group = "input")]
    pub expression: Option<String>,
    /// Arguments for the script to pass to the virtual machine.
    #[clap(short, long, action(ArgAction::Append))]
    pub arguments: Vec<String>,
}

impl ToString for RunCommand {
    fn to_string(&self) -> String {
        if let Some(expression) = self.expression.as_ref() {
            return expression.clone();
        }

        let mut buffer = String::new();
        let result = match self.path.as_ref() {
            None => stdin().read_to_string(&mut buffer),
            Some(path) => File::open(path)
                .expect(
                    format!(
                        "Unable to open file at {}.",
                        path.as_os_str().to_string_lossy()
                    )
                    .as_str(),
                )
                .read_to_string(&mut buffer),
        };

        result.expect("Unable to read input to a string.");

        buffer
    }
}

/// The sub-command to execute.
#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum Commands {
    Prompt(PromptCommand),
    Run(RunCommand),
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Prompt(PromptCommand)
    }
}
