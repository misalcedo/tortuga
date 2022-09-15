use crate::{CompilationError, SyntaxTree};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Validator {}

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationResult {
    Incomplete,
    Valid,
    Invalid(Vec<CompilationError>),
}

impl Validator {
    pub fn validate(&self, code: &str) -> ValidationResult {
        match SyntaxTree::try_from(code) {
            Ok(_) => ValidationResult::Valid,
            Err(errors) => {
                if errors.iter().any(|error| match error {
                    CompilationError::Syntax(error) => error.is_incomplete(),
                    _ => false,
                }) {
                    ValidationResult::Incomplete
                } else {
                    ValidationResult::Invalid(errors)
                }
            }
        }
        ValidationResult::Valid
    }
}
