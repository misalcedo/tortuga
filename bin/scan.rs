//! Performs Lexical Analysis on a file and prints the scanned sequence of tokens, their lexemes and attributes.

use crate::CommandLineError;
use colored::*;
use std::io::{stderr, stdout, Write};
use tortuga::compiler::Scanner;

pub fn scan_file(source: &str) -> Result<(), CommandLineError> {
    let mut std_out = stdout();
    let mut std_err = stderr();

    for result in Scanner::from(source) {
        match result {
            Ok(token) => {
                let kind = token.kind().to_string();
                let lexeme = token.lexeme().extract_from(source);
                let start = token.lexeme().start().to_string();

                writeln!(
                    std_out,
                    "- [{}] \"{}\" {} {}",
                    kind.green().bold(),
                    lexeme.blue(),
                    "@".yellow().bold(),
                    start.red()
                )?
            }
            Err(error) => writeln!(std_err, "{:?}", error)?,
        }
    }

    Ok(())
}
