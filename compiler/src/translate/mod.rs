use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod constant;
mod error;
mod number;
mod uri;
mod value;

use crate::grammar::{Expression, Internal, InternalKind, PostOrderIterator, Terminal, Uri};
use constant::Constants;
pub use error::TranslationError;
use value::Value;

pub struct Translator<'a, Iterator, Reporter> {
    reporter: Reporter,
    iterator: Iterator,
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

impl<'a, 'b, R> Translator<'a, PostOrderIterator<'a, 'b>, R>
where
    'a: 'b,
    R: ErrorReporter,
{
    fn new(program: &'b Program<'a>, reporter: R) -> Self {
        Translator {
            reporter,
            iterator: program.iter_post_order(),
            contexts: vec![Default::default()],
            code: Default::default(),
            functions: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
            stack: Default::default(),
            had_error: false,
        }
    }

    pub fn translate(mut self) -> Result<Executable, R> {
        if let Err(e) = self.simulate() {
            self.report_error(e);
        }

        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(Executable::new(
                self.code,
                self.functions,
                self.numbers,
                self.texts,
            ))
        }
    }

    fn simulate(&mut self) -> TranslationResult<()> {
        let mut previous_depth = 0;

        while let Some((depth, expression)) = self.iterator.next() {
            if previous_depth < depth {
                self.contexts.push(());
            } else if previous_depth > depth {
                self.contexts.pop().ok_or_else(|| {
                    TranslationError::from("Expected context stack to not be empty.")
                })?;
            }

            previous_depth = depth;

            self.simulate_expression(expression)?;
        }

        Ok(())
    }

    fn simulate_expression(&mut self, expression: &Expression<'a>) -> TranslationResult<()> {
        match expression {
            Expression::Internal(internal) => self.simulate_internal(internal),
            Expression::Terminal(terminal) => self.simulate_terminal(terminal),
        }
    }

    fn simulate_internal(&mut self, internal: &Internal) -> TranslationResult<()> {
        match internal.kind() {
            InternalKind::Block => {}
            InternalKind::Equality => {}
            InternalKind::Modulo => {}
            InternalKind::Subtract => {}
            InternalKind::Add => self.simulate_add()?,
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
        };

        Ok(())
    }

    fn simulate_add(&mut self) -> TranslationResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;

        match (a, b) {
            (Value::ConstantNumber(lhs), Value::ConstantNumber(rhs)) => {}
            (Value::Number, Value::Number) => {}
            (Value::ConstantNumber(lhs), Value::Number) => {}
            (Value::Number, Value::ConstantNumber(rhs)) => {}
            (lhs, rhs) => {}
        };

        Ok(())
    }

    fn simulate_terminal(&mut self, terminal: &Terminal<'a>) -> TranslationResult<()> {
        match terminal {
            Terminal::Identifier(identifier) => {
                todo!()
            }
            Terminal::Number(number) => {
                self.simulate_number(*number);
            }
            Terminal::Uri(uri) => {
                self.simulate_uri(*uri);
            }
        };

        Ok(())
    }

    fn simulate_uri(&mut self, uri: Uri<'a>) {
        let constant = match Text::try_from(uri) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Text::default()
            }
        };
        let index = self.texts.insert(constant, uri);

        if index >= u8::MAX as usize {
            self.report_error("Too many URI constants (max is 256).");
        }

        self.code.push(Operation::ConstantText(index as u8));
        self.stack.push(Value::ConstantText(index));
    }

    fn simulate_number(&mut self, number: grammar::Number<'a>) {
        let constant = match Number::try_from(number) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Number::default()
            }
        };
        let index = self.numbers.insert(constant, number);

        if index >= u8::MAX as usize {
            self.report_error("Too many number constants (max is 256).");
        }

        self.code.push(Operation::ConstantNumber(index as u8));
        self.stack.push(Value::ConstantNumber(index));
    }

    fn pop(&mut self) -> TranslationResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| TranslationError::from("Translation value stack is unexpectedly empty."))
    }

    fn report_error<E: Into<TranslationError>>(&mut self, error: E) {
        self.had_error = true;
        self.reporter.report_translation_error(error.into());
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
        let executable = Translator::new(&program, Vec::default()).translate()?;

        Ok(Translation {
            input: program,
            output: executable,
        })
    }
}
