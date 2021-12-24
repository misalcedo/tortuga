//! Terminal prompt reading and printing with editing and history.

use crate::CommandLineError;
use colored::*;
use rustyline::completion::Completer;
use rustyline::config::Config;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{error::ReadlineError, Editor, Helper};
use tortuga::{about, Interpreter, ParseError, Parser};
use tracing::error;

struct PromptHelper;

/// The prompt used to communicate with a user.
pub struct Prompt {
    line: usize,
    editor: Editor<PromptHelper>,
}

impl Default for Prompt {
    fn default() -> Self {
        let config = Config::builder()
            .auto_add_history(true)
            .tab_stop(2)
            .indent_size(2)
            .build();
        let mut editor = Editor::<PromptHelper>::with_config(config);

        editor.set_helper(Some(PromptHelper));

        Prompt { line: 1, editor }
    }
}

impl Prompt {
    /// Read input from the user via a terminal prompt.
    pub fn prompt(&mut self) -> Result<Option<String>, CommandLineError> {
        let prompt = format!("{}:{}> ", about::PROGRAM.green(), format!("{:03}", self.line).blue());

        match self.editor.readline(prompt.as_str()) {
            Ok(input) => {
                self.line += input.trim().lines().count();
                Ok(Some(input))
            }
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(ReadlineError::Eof) => Ok(None),
            Err(error) => Err(CommandLineError::PromptError(error)),
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

        match Parser::default().parse(ctx.input()) {
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
pub fn run_prompt() -> Result<(), CommandLineError> {
    let mut user = Prompt::default();
    let mut interpreter = Interpreter::default();

    println!("{} {}", about::PROGRAM.green(), about::VERSION);
    println!("{}", "Press Ctrl-C to exit.".yellow().bold());
    println!("");

    loop {
        match user.prompt()? {
            None => return Ok(()),
            Some(input) if input.trim().is_empty() => continue,
            Some(input) => {
                match Parser::default().parse(input.as_str()) {
                    Ok(program) => interpreter.interpret(&program),
                    Err(error) => error!("{}", error),
                };
            }
        }
    }
}
