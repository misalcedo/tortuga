//! Public interface of the tortuga compiler.

pub mod about;
mod errors;
mod grammar;
mod interpret;
mod lexer;
mod location;
mod number;
mod parser;
mod prompt;
mod scanner;
mod stream;
mod token;

pub use about::*;
pub use errors::TortugaError;
use interpret::Interpreter;
use lexer::Lexer;
use location::Location;
use parser::Parser;
use prompt::Prompt;
use scanner::Scanner;
use std::io::Write;
use tracing::error;

/// Runs a given string as a source file.
pub fn run(code: &str) -> Result<(), TortugaError> {
    let mut scanner = Scanner::from(code);
    let lexer = Lexer::new(&mut scanner);
    let parser = Parser::new(lexer);

    let mut interpreter = Interpreter::default();

    match parser.parse() {
        Ok(program) => interpreter.interpret(&program),
        Err(error) => error!("{}", error),
    }

    Ok(())
}

/// Run a prompt for the interpreter.
pub fn run_prompt() -> Result<(), TortugaError> {
    let mut user = Prompt::new();
    let mut start = Location::default();
    let mut interpreter = Interpreter::default();

    writeln!(std::io::stdout(), "{} {}\n", PROGRAM, VERSION)?;

    loop {
        match user.prompt(start.line())? {
            None => return Ok(()),
            Some(line) if line.trim().is_empty() => continue,
            Some(line) => {
                let mut scanner = Scanner::continue_from(start, line.as_str());
                let lexer = Lexer::new(&mut scanner);
                let parser = Parser::new(lexer);

                match parser.parse() {
                    Ok(program) => interpreter.interpret(&program),
                    Err(error) => error!("{}", error),
                };

                start = scanner.consume().unwrap_or_default();
            }
        }
    }
}
