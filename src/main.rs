mod about;
mod errors;
mod report;
mod scanner;
mod token;

use clap::{App, Arg, ArgMatches, SubCommand};
use errors::TortugaError;
use scanner::Scanner;
use std::fs;
use std::io::{stdin, stdout, Write};
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
                ),
        )
        .get_matches()
}

#[tracing::instrument]
fn run_subcommand(matches: ArgMatches<'_>) -> Result<(), TortugaError> {
    if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("input").map(Path::new).unwrap();
        let source = fs::read_to_string(input)?;

        run(source.as_str())
    } else {
        run_prompt(matches)
    }
}

#[tracing::instrument]
fn run(code: &str) -> Result<(), TortugaError> {
    let scanner = Scanner::new(code);

    for result in scanner {
        match result {
            Ok(token) => println!("Token: {:?}", token),
            Err(error) => report::print_lexical(code, error),
        }
    }

    println!("Reached the end of the file.");

    Ok(())
}

#[tracing::instrument]
fn run_prompt(matches: ArgMatches<'_>) -> Result<(), TortugaError> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        let line = buffer.trim();
        if line.is_empty() {
            continue;
        }

        if let Err(e) = run(line) {
            report::print(line, e);
        }
    }
}
