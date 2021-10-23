mod errors;
mod about;

use errors::TortugaError;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;

const APP_NAME: &str = env!("CARGO_BIN_NAME");

fn main() -> Result<(), TortugaError> {
    let matches = parse_arguments();

    set_verbosity(&matches)?;

    run_subcommand(matches)
}

fn set_verbosity(matches: &ArgMatches) -> Result<(), TortugaError> {
    let level = match matches.occurrences_of("verbosity") {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    LogTracer::init()?;

    let collector = tracing_subscriber::fmt()
        .with_max_level(level)
        .pretty()
        .finish();

    Ok(set_global_default(collector)?)
}

fn parse_arguments<'matches>() -> ArgMatches<'matches> {
    App::new(APP_NAME)
        .version(about::VERSION)
        .author(about::AUTHORS)
        .about(about::DESCRIPTION)
        .arg(
            Arg::with_name("verbosity")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs the specified input file.")
                .arg(
                    Arg::with_name("input")
                        .value_name("FILE")
                        .help("The input file to execute.")
                        .takes_value(true)
                        .index(1),
                )
        )
        .get_matches()
}

#[tracing::instrument]
fn run_subcommand(matches: ArgMatches<'_>) -> Result<(), TortugaError> {
    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("input").map(Path::new).unwrap();

        Ok(())
    } else {
        Err(errors::TortugaError::InvalidSubcommand(
            matches
                .subcommand_name()
                .map(String::from)
                .unwrap_or_else(Default::default),
        ))
    }
}
