use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod constant;
mod error;
mod number;
mod uri;

use constant::Constants;
pub use error::TranslationError;

pub struct Translator<'a, Reporter> {
    reporter: Reporter,
    program: Program<'a>,
    code: Vec<Operation>,
    functions: Constants<Function>,
    numbers: Constants<Number, grammar::Number<'a>>,
    texts: Constants<Text>,
    had_error: bool,
}

pub struct Translation<'a> {
    input: Program<'a>,
    output: Executable,
}

impl<'a, R> Translator<'a, R>
where
    R: ErrorReporter,
{
    fn new(program: Program<'a>, reporter: R) -> Self {
        Translator {
            reporter,
            program,
            code: Default::default(),
            functions: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
            had_error: false,
        }
    }

    pub fn translate(mut self) -> Result<Translation<'a>, R> {
        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(Translation {
                input: self.program,
                output: Executable::new(self.code, self.functions, self.numbers, self.texts),
            })
        }
    }
}

impl<'a> From<Translation<'a>> for Executable {
    fn from(translation: Translation<'a>) -> Self {
        translation.output
    }
}

impl<'a> TryFrom<&'a str> for Translation<'a> {
    type Error = Vec<CompilationError>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let program = Program::try_from(input)?;

        Translator::new(program, Vec::default()).translate()
    }
}
