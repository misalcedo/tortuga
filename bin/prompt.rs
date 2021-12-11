//! Terminal prompt reading and printing with editing and history.

use rustyline::completion::Completer;
use rustyline::config::Config;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{error::ReadlineError, Editor, Helper};
use std::io::Write;
use tortuga::{
    about, parse, Interpreter, Lexer, Location, ParseError, Parser, Scanner, TortugaError,
};
use tracing::error;

struct PromptHelper;

/// The prompt used to communicate with a user.
pub struct Prompt {
    editor: Editor<PromptHelper>,
}

impl Prompt {
    /// Create an instance of a `Prompt`.
    pub fn new() -> Self {
        let config = Config::builder()
            .auto_add_history(true)
            .tab_stop(2)
            .indent_size(2)
            .build();
        let mut editor = Editor::<PromptHelper>::with_config(config);

        editor.set_helper(Some(PromptHelper));

        Prompt { editor }
    }

    /// Read input from the user via a terminal prompt.
    pub fn prompt(&mut self, line: usize) -> Result<Option<String>, TortugaError> {
        let prompt = format!("{:03}> ", line);

        match self.editor.readline(prompt.as_str()) {
            Ok(line) => Ok(Some(line)),
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(ReadlineError::Eof) => Ok(None),
            Err(error) => Err(TortugaError::PromptError(Box::new(error))),
        }
    }
}

impl Helper for PromptHelper {}

impl Completer for PromptHelper {
    type Candidate = String;

    fn update(&self, _line: &mut LineBuffer, _start: usize, _elected: &str) {
        unreachable!()
    }
}

impl Highlighter for PromptHelper {}

impl Hinter for PromptHelper {
    type Hint = String;
}

impl Validator for PromptHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        if ctx.input().trim().is_empty() {
            return Ok(ValidationResult::Valid(None));
        }

        match parse(ctx.input()) {
            Ok(_) => Ok(ValidationResult::Valid(None)),
            Err(ParseError::EndOfFile) => Ok(ValidationResult::Incomplete),
            Err(error) => Ok(ValidationResult::Invalid(Some(format!(
                "\t{}",
                error.to_string()
            )))),
        }
    }
}

/// Runs the read-evaluate-print loop.
pub fn run_prompt() -> Result<(), TortugaError> {
    let mut user = Prompt::new();
    let mut start = Location::default();
    let mut interpreter = Interpreter::default();

    writeln!(std::io::stdout(), "{} {}\n", about::PROGRAM, about::VERSION)?;

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
