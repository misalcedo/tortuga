mod error;

use crate::grammar::Program;
use crate::CompilationError;
pub use error::AnalyticalError;

pub struct Analyzer<'a> {
    program: Program<'a>,
}

pub struct Analysis<'a> {
    program: Program<'a>,
}

impl<'a> TryFrom<&'a str> for Analysis<'a> {
    type Error = Vec<CompilationError>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Program::try_from(input).map(|p| Analysis { program: p })
    }
}
