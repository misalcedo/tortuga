//! Terminal prompt reading and printing with editing and history.

use crate::errors::{ParseError, TortugaError};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::scanner::Scanner;
use rustyline::completion::Completer;
use rustyline::config::Config;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{error::ReadlineError, Editor, Helper};

struct PromptHelper;

/// The prompt used to communicate with a user.
pub struct Prompt {
    line: u128,
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

        Prompt { line: 0, editor }
    }

    /// Read input from the user via a terminal prompt.
    pub fn prompt(&mut self) -> Result<Option<String>, TortugaError> {
        self.line += 1;

        let prompt = format!("{:03}> ", self.line);

        match self.editor.readline(prompt.as_str()) {
            Ok(line) => Ok(Some(line)),
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(ReadlineError::Eof) => Ok(None),
            Err(error) => Err(TortugaError::PromptError(error)),
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

        let mut scanner = Scanner::from(ctx.input());
        let lexer = Lexer::new(&mut scanner);
        let parser = Parser::new(lexer);

        match parser.parse() {
            Ok(_) => Ok(ValidationResult::Valid(None)),
            Err(ParseError::EndOfFile(_)) => Ok(ValidationResult::Incomplete),
            Err(error) => Ok(ValidationResult::Invalid(Some(format!(
                "\t{}",
                error.to_string()
            )))),
        }
    }
}
