use clap::{App, Arg, SubCommand};
use std::path::Path;
use tortuga::build;

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
                        .short("i")
                        .index(1)
                        .value_name("PATH")
                        .default_value("./src/")
                        .help("Sets a custom input directory for compilation.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .index(2)
                        .value_name("PATH")
                        .default_value("./out/")
                        .help("Sets a custom output directory for compilation.")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("build") {
        let output = matches.value_of("output").map(Path::new).unwrap();
        let input = matches.value_of("input").map(Path::new).unwrap();

        build(input, output);
    } else {
        println!(
            "Invalid subcommand name: {}",
            matches.subcommand_name().unwrap_or("")
        );
    }
}
