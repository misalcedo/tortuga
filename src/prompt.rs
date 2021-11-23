//! Terminal prompt reading and printing with editing and history.

use crate::errors::TortugaError;
use std::io::{stdin, stdout, Write};

/// The prompt used to communicate with a user.
pub struct Prompt {
    line: u128,
    buffer: String
}

impl Prompt {
    /// Create an instance of a `Prompt`.
    pub fn new() -> Self {
        Prompt { line: 0, buffer: String::new() }
    }

    /// Read input from the user via a terminal prompt.
    pub fn prompt(&mut self) -> Result<&str, TortugaError> {
        self.line += 1;
        self.buffer.clear();

        print!("{}> ", self.line);
        stdout().flush()?;

        stdin().read_line(&mut self.buffer)?;

        Ok(self.buffer.as_str().trim())
    }
}