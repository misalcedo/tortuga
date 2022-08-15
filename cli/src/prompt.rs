//! Terminal prompt reading and printing with editing and history.

use crate::about;
use crate::CommandLineError;
use colored::*;
use rustyline::completion::Completer;
use rustyline::config::Config;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{error::ReadlineError, Editor, Helper};
use std::io::{stderr, stdout, Write};
use tortuga_compiler::{
    CompilationError, ErrorReporter, LexicalError, Parser, Scanner, SyntaxError, Translation,
    TranslationError,
};
use tortuga_vm::VirtualMachine;
use tracing::error;

#[derive(Default)]
struct PromptHelper(Option<SyntaxError>, Vec<CompilationError>);

/// The prompt used to communicate with a user.
pub struct Prompt {
    line: usize,
    editor: Editor<PromptHelper>,
}

impl Prompt {
    fn new() -> Result<Self, CommandLineError> {
        let config = Config::builder()
            .auto_add_history(true)
            .tab_stop(2)
            .indent_size(2)
            .build();
        let mut editor = Editor::<PromptHelper>::with_config(config)?;

        editor.set_helper(Some(PromptHelper::default()));

        Ok(Prompt { line: 1, editor })
    }

    /// Read input from the user via a terminal prompt.
    pub fn prompt(&mut self) -> Result<Option<String>, CommandLineError> {
        let prompt = format!(
            "{}:{}> ",
            about::PROGRAM.green(),
            format!("{:03}", self.line).blue()
        );

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

impl ErrorReporter for PromptHelper {
    fn report_lexical_error(&mut self, error: LexicalError) {
        self.1.push(error.into());
    }

    fn report_syntax_error(&mut self, error: SyntaxError) {
        self.0 = Some(error.clone());
        self.1.push(error.into());
    }

    fn report_translation_error(&mut self, error: TranslationError) {
        self.1.push(error.into());
    }

    fn had_error(&self) -> bool {
        !self.1.is_empty()
    }
}

impl Validator for PromptHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        if ctx.input().trim().is_empty() {
            return Ok(ValidationResult::Valid(None));
        }

        let scanner = Scanner::from(ctx.input());
        let parser = Parser::new(scanner, PromptHelper::default());

        match parser.parse() {
            Ok(_) => Ok(ValidationResult::Valid(None)),
            Err(PromptHelper(Some(error), _)) if error.is_incomplete() => {
                Ok(ValidationResult::Invalid(Some(format!("\t{}", error))))
            }
            Err(_) => Ok(ValidationResult::Incomplete),
        }
    }
}

/// Runs the read-evaluate-print loop.
pub fn run_prompt() -> Result<(), CommandLineError> {
    let mut user = Prompt::new()?;
    let mut script = String::new();
    let mut machine = VirtualMachine::default();

    println!("{} {}", about::PROGRAM.green(), about::VERSION);
    println!("{}", "Press Ctrl-C to exit.".yellow().bold());
    println!();

    loop {
        match user.prompt()? {
            None => return Ok(()),
            Some(input) if input.trim().is_empty() => continue,
            Some(input) => {
                script.push_str(input.as_str());
                script.push('\n');

                match Translation::try_from(script.as_str()) {
                    Ok(translation) => {
                        machine.set_executable(translation);

                        match machine.run() {
                            Ok(Some(value)) => writeln!(stdout(), "=> {}", value)?,
                            Ok(None) => writeln!(stdout(), "=>")?,
                            Err(error) => writeln!(stderr(), "=> {}", error)?,
                        }
                    }
                    Err(error) => error!("{:?}", error),
                };
            }
        }
    }
}
