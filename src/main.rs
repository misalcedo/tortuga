use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;
use tortuga::{build, clean, TortugaError};
use tracing::{subscriber::set_global_default, Level};
use tracing_subscriber;

const APP_NAME: &str = env!("CARGO_BIN_NAME");

fn main() -> Result<(), TortugaError> {
    let matches = parse_arguments();

    set_verbosity(&matches)?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async { run_subcommand(matches).await })
}

fn set_verbosity(matches: &ArgMatches) -> Result<(), TortugaError> {
    let level = match matches.occurrences_of("verbosity") {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        3 | _ => Level::TRACE,
    };

    let collector = tracing_subscriber::fmt().with_max_level(level).finish();

    Ok(set_global_default(collector)?)
}

fn parse_arguments<'matches>() -> ArgMatches<'matches> {
    App::new(APP_NAME)
        .version(tortuga::about::VERSION)
        .author(tortuga::about::AUTHORS)
        .about(tortuga::about::DESCRIPTION)
        .arg(
            Arg::with_name("verbosity")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Compiles the input directory.")
                .arg(
                    Arg::with_name("input")
                        .long("input")
                        .short("i")
                        .value_name("PATH")
                        .default_value("src")
                        .help("Sets a custom input directory for compilation.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .value_name("PATH")
                        .default_value("out")
                        .help("Sets a custom output directory for compilation.")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Cleans the output directory.")
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .value_name("PATH")
                        .default_value("out")
                        .help("Sets a custom output directory for cleaning.")
                        .takes_value(true),
                ),
        )
        .get_matches()
}

#[tracing::instrument]
async fn run_subcommand(matches: ArgMatches<'_>) -> Result<(), TortugaError> {
    if let Some(matches) = matches.subcommand_matches("build") {
        let output = matches.value_of("output").map(Path::new).unwrap();
        let input = matches.value_of("input").map(Path::new).unwrap();

        let results = build(input, output).await;
        let errors: Vec<TortugaError> = results
            .into_iter()
            .filter(Result::is_err)
            .map(Result::unwrap_err)
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(TortugaError::Build(errors))
        }
    } else if let Some(matches) = matches.subcommand_matches("clean") {
        let output = matches.value_of("output").map(Path::new).unwrap();

        clean(output).await
    } else {
        Err(tortuga::TortugaError::InvalidSubcommand(
            matches
                .subcommand_name()
                .map(String::from)
                .unwrap_or_else(Default::default),
        ))
    }
}
