//! Analyses a program for the following:
//! - Unused variables.
//! - Define a variable before using.
//! - Type checking.

mod capture;
mod constants;
mod context;
mod error;
mod local;

use crate::Program;
use crate::{CompilationError, ErrorReporter};
use constants::ConstantAnalysis;
use context::ScopeContext;
pub use error::AnalyticalError;
use std::fmt::{Display, Formatter};

pub struct Analyzer<'a, Reporter> {
    program: Program<'a>,
    reporter: Reporter,
    had_error: bool,
    scopes: Vec<ScopeContext<'a>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Analysis<'a> {
    program: Program<'a>,
    constants: ConstantAnalysis<'a>,
}

type AnalyticalResult<Output> = Result<Output, AnalyticalError>;

impl<'a, R> Analyzer<'a, R>
where
    R: ErrorReporter,
{
    fn new(program: Program<'a>, reporter: R) -> Self {
        Analyzer {
            program,
            reporter,
            had_error: false,
            scopes: Default::default(),
        }
    }

    pub fn analyze(mut self) -> Result<Analysis<'a>, R> {
        let constants = ConstantAnalysis::from(&self.program);

        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(Analysis {
                program: self.program,
                constants,
            })
        }
    }
}

impl<'a> TryFrom<&'a str> for Analysis<'a> {
    type Error = Vec<CompilationError>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let program = Program::try_from(input)?;

        Analyzer::new(program, Vec::default()).analyze()
    }
}

impl<'a> Display for Analysis<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# Program Analysis")?;
        writeln!(f, "{}", self.constants)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undefined_variable() {
        assert!(!Analysis::try_from("x + 42").unwrap_err().is_empty());
    }

    #[test]
    fn add_wrong_types() {
        assert!(!Analysis::try_from("x = \"Hello\"\nx + 42")
            .unwrap_err()
            .is_empty());
    }

    #[test]
    fn undefined() {
        assert!(
            !Analysis::try_from(include_str!("../../../examples/undefined.ta"))
                .unwrap_err()
                .is_empty()
        );
    }

    #[test]
    fn factorial() {
        let analysis = Analysis::try_from(include_str!("../../../examples/factorial.ta")).unwrap();

        assert!(false);
    }
}
