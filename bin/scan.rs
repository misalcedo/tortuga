//! Performs Lexical Analysis on a file and prints the scanned sequence of tokens, their lexemes and attributes.

use crate::CommandLineError;
use colored::*;
use std::io::{stderr, stdout, Write};
use tortuga::compiler::{Kind, Scanner};
use tortuga::runtime::Number;

pub fn scan_file(source: &str) -> Result<(), CommandLineError> {
    let mut std_out = stdout();
    let mut std_err = stderr();

    for (index, result) in Scanner::from(source).enumerate() {
        match result {
            Ok(token) => {
                let kind = token.kind().to_string();
                let lexeme = token.lexeme().extract_from(source);
                let start = token.lexeme().start().to_string();

                match token.kind() {
                    Kind::Number => writeln!(
                        std_out,
                        "{}) [{}] \"{}\" = {} {} {}",
                        index + 1,
                        kind.green().bold(),
                        lexeme.blue(),
                        lexeme
                            .parse::<Number>()
                            .unwrap_or_default()
                            .to_string()
                            .blue()
                            .bold(),
                        "@".yellow().bold(),
                        start.red()
                    )?,
                    Kind::Identifier => writeln!(
                        std_out,
                        "{}) [{}] \"{}\" {} {}",
                        index + 1,
                        kind.green().bold(),
                        lexeme.blue(),
                        "@".yellow().bold(),
                        start.red()
                    )?,
                    _ => writeln!(
                        std_out,
                        "{}) [{}] {} {}",
                        index + 1,
                        kind.green().bold(),
                        "@".yellow().bold(),
                        start.red()
                    )?,
                }
            }
            Err(error) => {
                let kind = error.kind().to_string();
                let lexeme = error.lexeme().extract_from(source);
                let start = error.lexeme().start().to_string();

                writeln!(
                    std_err,
                    "{}) [{}|{}] \"{}\" {} {}",
                    index + 1,
                    "ERROR".red().bold(),
                    kind.green().bold(),
                    lexeme.blue(),
                    "@".yellow().bold(),
                    start.red()
                )?
            }
        }
    }

    Ok(())
}
