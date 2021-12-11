//! Public interface of the tortuga compiler.

pub mod about;
mod compile;
mod errors;
pub mod grammar;
mod interpret;
mod prompt;

pub use about::*;
pub use compile::parse;
pub use errors::TortugaError;
use compile::{Lexer, Location, Parser, Scanner};
use interpret::Interpreter;
use prompt::Prompt;
use std::io::Write;
use tracing::error;

/// Runs a given string as a source file.
/// 
/// # Examples
/// ```rust
/// use tortuga::run;
/// 
/// run("2#10^2").unwrap();
/// ```
pub fn run(code: &str) -> Result<(), TortugaError> {
    let mut interpreter = Interpreter::default();

    match parse(code) {
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
