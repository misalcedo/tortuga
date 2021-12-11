use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::Path;
use tortuga::TortugaError;
use tracing::{subscriber::set_global_default, Level};
use tracing_log::LogTracer;

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

    let collector = tracing_subscriber::fmt().with_max_level(level).finish();

    Ok(set_global_default(collector)?)
}

fn parse_arguments<'matches>() -> ArgMatches {
    App::new(tortuga::PROGRAM)
        .version(tortuga::VERSION)
        .author(tortuga::AUTHORS)
        .about(tortuga::DESCRIPTION)
        .arg(
            Arg::new("verbosity")
                .long("verbose")
                .short('v')
                .multiple_occurrences(true)
                .help("Sets the level of verbosity."),
        )
        .subcommand(
            App::new("run")
                .about("Runs the specified input file.")
                .arg(
                    Arg::new("input")
                        .value_name("FILE")
                        .required(true)
                        .help("The input file to execute.")
                        .takes_value(true)
                        .index(1),
                ),
        )
        .get_matches()
}

fn run_subcommand(matches: ArgMatches) -> Result<(), TortugaError> {
    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches
            .value_of("input")
            .map(Path::new)
            .expect("Missing required field INPUT.");
        let source = fs::read_to_string(input)?;

        tortuga::run(source.as_str())
    } else {
        tortuga::run_prompt()
    }
}
