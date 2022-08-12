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
    numbers: IndexedSet<grammar::Number<'a>, Number>,
    texts: IndexedSet<Uri<'a>, Text>,
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
            functions: IndexedSet::from([Function::default()]),
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

        self.update_entrypoint()
    }

    fn update_entrypoint(&mut self) -> TranslationResult<()> {
        let root = self
            .contexts
            .pop()
            .ok_or_else(|| TranslationError::from("Expected function context to not be empty."))?;
        let script = self
            .functions
            .get_mut(0)
            .ok_or_else(|| TranslationError::from("Expected script function to be present."))?;

        script.set_locals(root.locals());

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
        let value = self.pop()?;
        let assignee = self.pop()?;

        match assignee {
            Value::Uninitialized(index) => {
                let depth = self.contexts.len();
                let scope = self.contexts.last_mut().ok_or_else(|| {
                    TranslationError::from("Expected function contexts to not be empty.")
                })?;
                let local = scope.local_mut(index).ok_or_else(|| {
                    TranslationError::from("Unable to find local in current scope.")
                })?;

                local.initialize(depth, value);

                self.code.push(Operation::SetLocal(local.offset() as u8));
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
                self.code.push(operation);
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
            Terminal::Identifier(identifier) => self.simulate_identifier(identifier)?,
            Terminal::Number(number) => self.simulate_number(*number),
            Terminal::Uri(uri) => self.simulate_uri(*uri),
        };

        Ok(())
    }

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) -> TranslationResult<()> {
        let scope = self
            .contexts
            .last_mut()
            .ok_or_else(|| TranslationError::from("Expected function contexts to not be empty."))?;

        match scope.resolve_local(identifier) {
            Some(local) => {
                self.code.push(Operation::GetLocal(local.offset() as u8));
                self.stack.push(local.kind().clone());
                Ok(())
            }
            None => {
                let index = scope.add_local(*identifier);

                if index >= u8::MAX as usize {
                    self.report_error("Too many locals (max is 256).");
                }

                self.stack.push(Value::Uninitialized(index));
                Ok(())
            }
        }
    }

    fn simulate_uri(&mut self, uri: Uri<'a>) {
        let constant = match Text::try_from(uri) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Text::default()
            }
        };
        let index = self.texts.insert(uri, constant);

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
        let index = self.numbers.insert(number, constant);

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

    #[test]
    fn undefined_variable() {
        assert_eq!(Translation::try_from("x + 40").unwrap_err().len(), 1);
    }

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
    fn add_with_variable() {
        let executable: Executable = Translation::try_from("x = 2\nx + 40").unwrap().into();

        assert_eq!(
            executable.code(0, executable.len()),
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
            Some(&Function::new(0, 1, Vec::default()))
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
