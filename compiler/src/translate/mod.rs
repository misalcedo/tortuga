use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod constant;
mod error;
mod number;
mod uri;
mod value;

use crate::grammar::{Expression, InternalKind, Terminal, Uri, WithoutScopeDepth};
use constant::Constants;
pub use error::TranslationError;
use value::Value;

pub struct Translator<'a, Reporter> {
    reporter: Reporter,
    program: Program<'a>,
    contexts: Vec<()>,
    code: Vec<Operation>,
    functions: Constants<Function>,
    numbers: Constants<Number, grammar::Number<'a>>,
    texts: Constants<Text, Uri<'a>>,
    stack: Vec<Value>,
    had_error: bool,
}

pub struct Translation<'a> {
    input: Program<'a>,
    output: Executable,
}

type TranslationResult<Output> = Result<Output, TranslationError>;

impl<'a, R> Translator<'a, R>
where
    R: ErrorReporter,
{
    fn new(program: Program<'a>, reporter: R) -> Self {
        Translator {
            reporter,
            program,
            contexts: vec![Default::default()],
            code: Default::default(),
            functions: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
            stack: Default::default(),
            had_error: false,
        }
    }

    pub fn translate(mut self) -> Result<Translation<'a>, R> {
        if let Err(e) = self.simulate() {
            self.had_error = true;
            self.reporter.report_translation_error(e);
        }

        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(Translation {
                input: self.program,
                output: Executable::new(self.code, self.functions, self.numbers, self.texts),
            })
        }
    }

    fn simulate(&mut self) -> TranslationResult<()> {
        let previous_depth = 0;

        for (depth, expression) in self.program.iter_post_order() {
            if previous_depth < depth {
                self.contexts.push(());
            } else if previous_depth > depth {
                self.contexts.pop().ok_or_else(|| {
                    TranslationError::from("Expected context stack to not be empty.")
                })?;
            }

            match expression {
                Expression::Internal(internal) => match internal.kind() {
                    InternalKind::Block => {}
                    InternalKind::Equality => {}
                    InternalKind::Modulo => {}
                    InternalKind::Subtract => {}
                    InternalKind::Add => {}
                    InternalKind::Divide => {}
                    InternalKind::Multiply => {}
                    InternalKind::Power => {}
                    InternalKind::Call => {}
                    InternalKind::Grouping => {}
                    InternalKind::Condition => {}
                    InternalKind::Inequality => {}
                    InternalKind::LessThan => {}
                    InternalKind::GreaterThan => {}
                    InternalKind::LessThanOrEqualTo => {}
                    InternalKind::GreaterThanOrEqualTo => {}
                },
                Expression::Terminal(terminal) => match terminal {
                    Terminal::Identifier(identifier) => {
                        todo!()
                    }
                    Terminal::Number(number) => {
                        let constant = match Number::try_from(*number) {
                            Ok(c) => c,
                            Err(e) => {
                                self.had_error = true;
                                self.reporter
                                    .report_translation_error(TranslationError::from(e))
                            }
                        };
                        let index = self.numbers.insert(constant, *number);

                        self.stack.push(Value::ConstantNumber(index));
                    }
                    Terminal::Uri(uri) => {
                        let constant = match Text::try_from(*uri) {
                            Ok(c) => c,
                            Err(e) => {
                                self.had_error = true;
                                self.reporter
                                    .report_translation_error(TranslationError::from(e))
                            }
                        };
                        let index = self.texts.insert(constant, *uri);

                        self.stack.push(Value::ConstantText(index));
                    }
                },
            };
        }

        Ok(())
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
