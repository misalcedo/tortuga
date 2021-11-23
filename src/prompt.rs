//! Terminal prompt reading and printing with editing and history.

use crate::errors::TortugaError;
use rustyline::{Editor, error::ReadlineError};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};

/// The prompt used to communicate with a user.
pub struct Prompt {
    line: u128,
    editor: Editor<()>
}

impl Prompt {
    /// Create an instance of a `Prompt`.
    pub fn new() -> Self {
        Prompt { line: 0, editor: Editor::<()>::new() }
    }

    /// Read input from the user via a terminal prompt.
    pub fn prompt(&mut self) -> Result<Option<String>, TortugaError> {
        self.line += 1;

        let prompt = format!("{}> ", self.line);

        match self.editor.readline(prompt.as_str()) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(Some(line))
            },
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(ReadlineError::Eof) => Ok(None),
            Err(error) => Err(TortugaError::PromptError(error))
        }
    }
}

impl Validator for Prompt {
    fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}