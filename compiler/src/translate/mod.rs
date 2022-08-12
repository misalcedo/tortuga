use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod capture;
mod context;
mod error;
mod indices;
mod local;
mod number;
mod uri;
mod value;

use crate::grammar::{
    Expression, Identifier, Internal, InternalKind, PostOrderIterator, Terminal, Uri,
};
use crate::translate::context::ScopeContext;
pub use error::TranslationError;
use indices::IndexedSet;
use value::Value;

pub struct Translator<'a, Iterator, Reporter> {
    reporter: Reporter,
    iterator: Iterator,
    contexts: Vec<ScopeContext<'a>>,
    code: Vec<Operation>,
    functions: IndexedSet<Function>,
    numbers: IndexedSet<Number, grammar::Number<'a>>,
    texts: IndexedSet<Text, Uri<'a>>,
    stack: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn new(program: &'b Program<'a>, reporter: R) -> Self {
        Translator {
            reporter,
            iterator: program.iter_post_order(),
            contexts: vec![Default::default()],
            code: Default::default(),
            functions: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
            stack: Default::default(),
        }
    }

    pub fn translate(mut self) -> Result<Executable, R> {
        if let Err(e) = self.simulate() {
            self.report_error(e);
        }

        if self.reporter.had_error() {
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
                self.contexts.push(ScopeContext::default());
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
            InternalKind::Equality => self.simulate_equality()?,
            InternalKind::Modulo => self.simulate_binary(Operation::Remainder)?,
            InternalKind::Subtract => self.simulate_binary(Operation::Subtract)?,
            InternalKind::Add => self.simulate_binary(Operation::Add)?,
            InternalKind::Divide => self.simulate_binary(Operation::Divide)?,
            InternalKind::Multiply => self.simulate_binary(Operation::Multiply)?,
            InternalKind::Power => todo!(),
            InternalKind::Call => {}
            InternalKind::Grouping => {}
            InternalKind::Condition => {}
            InternalKind::Inequality => self.simulate_negated_binary(Operation::Equal)?,
            InternalKind::LessThan => self.simulate_binary(Operation::Less)?,
            InternalKind::GreaterThan => self.simulate_binary(Operation::Greater)?,
            InternalKind::LessThanOrEqualTo => self.simulate_negated_binary(Operation::Greater)?,
            InternalKind::GreaterThanOrEqualTo => self.simulate_negated_binary(Operation::Less)?,
        };

        Ok(())
    }

    fn simulate_equality(&mut self) -> TranslationResult<()> {
        Ok(())
    }

    fn simulate_call(&mut self) -> TranslationResult<()> {
        Ok(())
    }

    fn simulate_condition(&mut self) -> TranslationResult<()> {
        Ok(())
    }

    fn simulate_binary(&mut self, operation: Operation) -> TranslationResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;

        match (a, b) {
            (Value::Number(_), Value::Number(_)) => {
                self.code.push(operation);
                self.stack.push(Value::Number(None));
            }
            (Value::Any, Value::Any) => self.stack.push(Value::Any),
            (_, _) => {
                self.report_error("Operands must be numbers.");
                self.stack.push(Value::Any);
            }
        };

        Ok(())
    }

    fn simulate_negated_binary(&mut self, operation: Operation) -> TranslationResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;

        match (a, b) {
            (Value::Number(_), Value::Number(_)) => {
                self.code.push(Operation::Not);
                self.code.push(operation);
                self.stack.push(Value::Number(None));
            }
            (Value::Any, Value::Any) => self.stack.push(Value::Any),
            (_, _) => {
                self.report_error("Operands must be numbers.");
                self.stack.push(Value::Any);
            }
        };

        Ok(())
    }

    fn get_number(&mut self, index: usize) -> TranslationResult<Number> {
        self.numbers
            .get(index)
            .copied()
            .ok_or_else(|| TranslationError::from("Invalid index for number constant."))
    }

    fn simulate_terminal(&mut self, terminal: &Terminal<'a>) -> TranslationResult<()> {
        match terminal {
            Terminal::Identifier(identifier) => self.simulate_identifier(identifier),
            Terminal::Number(number) => {
                self.simulate_number(*number);
            }
            Terminal::Uri(uri) => {
                self.simulate_uri(*uri);
            }
        };

        Ok(())
    }

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) {
        self.stack.push(Value::Unknown);
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
        self.stack.push(Value::Text(Some(index)));
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
        self.stack.push(Value::Number(Some(index)));
    }

    fn pop(&mut self) -> TranslationResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| TranslationError::from("Translation value stack is unexpectedly empty."))
    }

    fn report_error<E: Into<TranslationError>>(&mut self, error: E) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tortuga_executable::OperationCode;

    // #[test]
    // fn undefined_variable() {
    //     assert!(!Analysis::try_from("x + 42").unwrap_err().is_empty());
    // }

    #[test]
    fn add_numbers() {
        let executable: Executable = Translation::try_from("2 + 40").unwrap().into();

        assert_eq!(
            executable.code(0, executable.len()),
            &[
                OperationCode::ConstantNumber as u8,
                0,
                OperationCode::ConstantNumber as u8,
                1,
                OperationCode::Add as u8
            ]
        );
    }

    #[test]
    fn add_wrong_types() {
        assert_eq!(
            Translation::try_from("\"Hello\" + 42").unwrap_err().len(),
            1
        );
    }

    // #[test]
    // fn undefined() {
    //     assert!(
    //         !Analysis::try_from(include_str!("../../../examples/undefined.ta"))
    //             .unwrap_err()
    //             .is_empty()
    //     );
    // }
    //
    // #[test]
    // fn factorial() {
    //     let analysis = Analysis::try_from(include_str!("../../../examples/factorial.ta")).unwrap();
    //
    //     assert!(false);
    // }
}
