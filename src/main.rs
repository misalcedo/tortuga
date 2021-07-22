use clap::{App, Arg, SubCommand};
use std::path::Path;
use tortuga::{build, clean, TortugaError};

const APP_NAME: &str = env!("CARGO_BIN_NAME");

#[tokio::main]
async fn main() -> Result<(), TortugaError> {
    let matches = App::new(APP_NAME)
        .version(tortuga::about::VERSION)
        .author(tortuga::about::AUTHORS)
        .about(tortuga::about::DESCRIPTION)
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
        .get_matches();

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
