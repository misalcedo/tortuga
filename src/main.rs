use clap::{App, Arg, SubCommand};
use std::path::Path;
use tortuga::{build, clean};

fn main() {
    let matches = App::new("Tortuga")
        .version("0.2.0")
        .author("Miguel D. Salcedo <miguel@salcedo.cc>")
        .about("Compiler and runtime executable for the Tortuga programming language.")
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

        build(input, output).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("clean") {
        let output = matches.value_of("output").map(Path::new).unwrap();

        clean(output).unwrap();
    } else {
        println!(
            "Invalid subcommand name: {}",
            matches.subcommand_name().unwrap_or("")
        );
    }
}
