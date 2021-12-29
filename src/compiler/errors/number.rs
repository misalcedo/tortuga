//! Error parsing a numerical literal into a value.

/// An error that occurred parsing a numerical literal into runtime value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseNumberError {
    lexeme: String,
}

impl ParseNumberError {
    /// The number literal that could not be parsed.
    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }
}

impl From<&str> for ParseNumberError {
    fn from(lexeme: &str) -> Self {
        ParseNumberError {
            lexeme: lexeme.to_string(),
        }
    }
}
