use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use std::mem;
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod capture;
mod context;
mod error;
mod indices;
mod local;
mod number;
mod uri;
mod value;

use crate::grammar::{ExpressionKind, Identifier, Iter, Node, Uri};
use context::ScopeContext;
pub use error::TranslationError;
use indices::IndexedSet;
use value::Value;

pub struct Translator<'a, Iterator, Reporter> {
    reporter: Reporter,
    iterator: Iterator,
    context: ScopeContext<'a>,
    contexts: Vec<ScopeContext<'a>>,
    stack: Vec<Value>,
    functions: Vec<Function>,
    numbers: IndexedSet<grammar::Number<'a>, Number>,
    texts: IndexedSet<Uri<'a>, Text>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Translation<'a> {
    input: Program<'a>,
    output: Executable,
}

type TranslationResult<Output> = Result<Output, TranslationError>;

impl<'a, 'b, R> Translator<'a, Iter<'a, 'b>, R>
where
    'a: 'b,
    R: ErrorReporter,
{
    pub fn new(program: &'b Program<'a>, reporter: R) -> Self {
        Translator {
            reporter,
            iterator: program.iter(),
            context: Default::default(),
            contexts: Default::default(),
            functions: Default::default(),
            stack: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
        }
    }

    pub fn translate(mut self) -> Result<Executable, R> {
        if let Err(e) = self.simulate() {
            self.report_error(e);
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(Executable::new(self.functions, self.numbers, self.texts))
        }
    }

    fn simulate(&mut self) -> TranslationResult<()> {
        while let Some(node) = self.iterator.next() {
            self.simulate_expression(node)?;
        }

        self.update_entrypoint()
    }

    // TODO: Figure out how to denote the number of locals in the script.
    fn update_entrypoint(&mut self) -> TranslationResult<()> {
        let mut context = ScopeContext::default();

        mem::swap(&mut self.context, &mut context);

        self.functions.push(Function::from(context));

        Ok(())
    }

    fn simulate_expression(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
        match node.expression().kind() {
            ExpressionKind::Block if node.discovered() => {}
            ExpressionKind::Block => {}
            ExpressionKind::Equality => self.simulate_equality()?,
            ExpressionKind::Modulo => self.simulate_binary(Operation::Remainder)?,
            ExpressionKind::Subtract => self.simulate_binary(Operation::Subtract)?,
            ExpressionKind::Add => self.simulate_binary(Operation::Add)?,
            ExpressionKind::Divide => self.simulate_binary(Operation::Divide)?,
            ExpressionKind::Multiply => self.simulate_binary(Operation::Multiply)?,
            ExpressionKind::Power => self.simulate_binary(Operation::Power)?,
            ExpressionKind::Call => {}
            ExpressionKind::Grouping => {}
            ExpressionKind::Condition => {}
            ExpressionKind::Inequality => self.simulate_negated_binary(Operation::Equal)?,
            ExpressionKind::LessThan => self.simulate_binary(Operation::Less)?,
            ExpressionKind::GreaterThan => self.simulate_binary(Operation::Greater)?,
            ExpressionKind::LessThanOrEqualTo => {
                self.simulate_negated_binary(Operation::Greater)?
            }
            ExpressionKind::GreaterThanOrEqualTo => {
                self.simulate_negated_binary(Operation::Less)?
            }
            ExpressionKind::Number(number) => self.simulate_number(number),
            ExpressionKind::Identifier(identifier) => self.simulate_identifier(identifier)?,
            ExpressionKind::Uri(uri) => self.simulate_uri(uri),
        };

        Ok(())
    }

    //     match terminal {
    //     Expression::from(identifier) => ,
    //     Terminal::Number(number) => ,
    //     Terminal::Uri(uri) => ,
    // };

    fn simulate_equality(&mut self) -> TranslationResult<()> {
        let value = self.pop()?;
        let assignee = self.pop()?;

        match assignee {
            Value::Uninitialized(index) => {
                let depth = self.contexts.len();
                let local = self
                    .context
                    .local_mut(index)
                    .ok_or_else(|| {
                        TranslationError::from("Unable to find local in current scope.")
                    })?
                    .initialize(depth, value);

                self.context.add_operation(Operation::SetLocal(local as u8));
                self.stack.push(value);
            }
            Value::Any => {}
            Value::Closure => {}
            Value::Number(_) => {}
            Value::Text(_) => {}
        }

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
                self.context.add_operation(operation);
                self.stack.push(Value::Number(None));
            }
            (Value::Any, Value::Any) => self.stack.push(Value::Any),
            (lhs, rhs) => {
                self.report_error(format!("Operands must be numbers: {}, {}.", lhs, rhs));
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
                self.context.add_operation(Operation::Not);
                self.context.add_operation(operation);
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

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) -> TranslationResult<()> {
        match self.context.resolve_local(identifier) {
            Some(local) => {
                self.context
                    .add_operation(Operation::GetLocal(local.offset() as u8));
                self.stack.push(local.kind().clone());
                Ok(())
            }
            None => {
                let index = self.context.add_local(*identifier);

                if index >= u8::MAX as usize {
                    self.report_error("Too many locals (max is 256).");
                }

                self.stack.push(Value::Uninitialized(index));
                Ok(())
            }
        }
    }

    fn simulate_uri(&mut self, uri: &Uri<'a>) {
        let constant = match Text::try_from(*uri) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Text::default()
            }
        };
        let index = self.texts.insert(*uri, constant);

        if index >= u8::MAX as usize {
            self.report_error("Too many URI constants (max is 256).");
        }

        self.context
            .add_operation(Operation::ConstantText(index as u8));
        self.stack.push(Value::Text(Some(index)));
    }

    fn simulate_number(&mut self, number: &grammar::Number<'a>) {
        let constant = match Number::try_from(*number) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Number::default()
            }
        };
        let index = self.numbers.insert(*number, constant);

        if index >= u8::MAX as usize {
            self.report_error("Too many number constants (max is 256).");
        }

        self.context
            .add_operation(Operation::ConstantNumber(index as u8));
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

    #[test]
    fn undefined_variable() {
        assert_eq!(Translation::try_from("x + 40").unwrap_err().len(), 1);
    }

    #[test]
    fn add_numbers() {
        let executable: Executable = Translation::try_from("2 + 40").unwrap().into();

        assert_eq!(
            executable.function(0).unwrap().code().as_slice(),
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
    fn add_with_variable() {
        let executable: Executable = Translation::try_from("x = 2\nx + 40").unwrap().into();

        assert_eq!(
            executable.function(0).unwrap().code().as_slice(),
            &[
                OperationCode::ConstantNumber as u8,
                0,
                OperationCode::SetLocal as u8,
                1,
                OperationCode::GetLocal as u8,
                1,
                OperationCode::ConstantNumber as u8,
                1,
                OperationCode::Add as u8
            ]
        );
        assert_eq!(
            executable.function(0),
            Some(&Function::new(
                0,
                1,
                vec![0, 0, 4, 1, 5, 1, 0, 1, 11],
                vec![]
            ))
        );
    }

    #[test]
    fn add_wrong_types() {
        assert_eq!(
            Translation::try_from("\"Hello\" + 42").unwrap_err().len(),
            1
        );
    }

    #[test]
    fn undefined() {
        assert_eq!(
            Translation::try_from(include_str!("../../../examples/undefined.ta"))
                .unwrap_err()
                .len(),
            1
        );
    }

    #[test]
    fn factorial() {
        let executable: Executable =
            Translation::try_from(include_str!("../../../examples/factorial.ta"))
                .unwrap()
                .into();

        assert!(false);
    }
}
