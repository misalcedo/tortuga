use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use std::mem;
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod capture;
mod context;
mod error;
mod function;
mod indices;
mod local;
mod number;
mod uri;
mod value;

use crate::grammar::{ExpressionKind, Identifier, Iter, Node, Uri};
use crate::translate::error::ErrorKind;
use context::ScopeContext;
pub use error::TranslationError;
use function::TypedFunction;
use indices::IndexedSet;
use value::Value;

pub struct Translator<'a, Iterator, Reporter> {
    reporter: Reporter,
    iterator: Iterator,
    context: ScopeContext<'a>,
    contexts: Vec<ScopeContext<'a>>,
    stack: Vec<Value>,
    functions: Vec<TypedFunction>,
    numbers: IndexedSet<grammar::Number<'a>, Number>,
    texts: IndexedSet<Uri<'a>, Text>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Translation<'a> {
    input: Program<'a>,
    output: Executable,
}

type TranslationResult<Output> = Result<Output, TranslationError>;
static UNDISCOVERED_KINDS: &[ExpressionKind] = &[ExpressionKind::Block];

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

    // TODO:
    // * Prevents blocks outside of assignment.
    // * Prevent local variable in an assignment that is not a block.
    // * Check function type signatures.
    pub fn translate(mut self) -> Result<Executable, R> {
        if let Err(e) = self.simulate() {
            self.report_error(e);
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            let functions: Vec<Function> = self.functions.into_iter().map(Function::from).collect();

            Ok(Executable::new(functions, self.numbers, self.texts))
        }
    }

    fn simulate(&mut self) -> TranslationResult<()> {
        while let Some(node) = self.next_node() {
            self.simulate_expression(node)?;
        }

        self.simulate_program()
    }

    fn next_node(&mut self) -> Option<Node<'a, 'b>> {
        let mut node = self.iterator.next()?;

        while !node.discovered() && !UNDISCOVERED_KINDS.contains(node.expression().kind()) {
            node = self.iterator.next()?;
        }

        Some(node)
    }

    // TODO: Figure out how to denote the number of locals in the script.
    fn simulate_program(&mut self) -> TranslationResult<()> {
        let mut context = ScopeContext::default();

        mem::swap(&mut self.context, &mut context);

        if !self.contexts.is_empty() {
            self.report_error(ErrorKind::ExpectedEndOfBlock);
        }

        let function = Function::from(context);
        self.functions
            .push(TypedFunction::new(function, vec![], vec![]));

        Ok(())
    }

    fn simulate_expression(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
        // TODO: Remove
        println!("Node: {:?}:{}", node.expression().kind(), node.discovered());

        match node.expression().kind() {
            ExpressionKind::Block => self.simulate_block(node),
            ExpressionKind::Equality => self.simulate_equality(node),
            ExpressionKind::Modulo => self.simulate_binary(Operation::Remainder),
            ExpressionKind::Subtract => self.simulate_binary(Operation::Subtract),
            ExpressionKind::Add => self.simulate_binary(Operation::Add),
            ExpressionKind::Divide => self.simulate_binary(Operation::Divide),
            ExpressionKind::Multiply => self.simulate_binary(Operation::Multiply),
            ExpressionKind::Power => self.simulate_binary(Operation::Power),
            ExpressionKind::Call => self.simulate_call(node),
            ExpressionKind::Grouping => self.simulate_grouping(node),
            ExpressionKind::Condition => self.simulate_condition(node),
            ExpressionKind::Inequality => self.simulate_negated_binary(Operation::Equal),
            ExpressionKind::LessThan => self.simulate_binary(Operation::Less),
            ExpressionKind::GreaterThan => self.simulate_binary(Operation::Greater),
            ExpressionKind::LessThanOrEqualTo => self.simulate_negated_binary(Operation::Greater),
            ExpressionKind::GreaterThanOrEqualTo => self.simulate_negated_binary(Operation::Less),
            ExpressionKind::Number(number) => self.simulate_number(number),
            ExpressionKind::Identifier(identifier) => self.simulate_identifier(identifier),
            ExpressionKind::Uri(uri) => self.simulate_uri(uri),
        }
    }

    fn simulate_block(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
        if node.discovered() {
            let mut context = self
                .contexts
                .pop()
                .ok_or_else(|| TranslationError::from(ErrorKind::EmptyContexts))?;

            mem::swap(&mut context, &mut self.context);

            let function = Function::from(context);
            let index = self.functions.len();

            self.functions
                .push(TypedFunction::new(function, vec![], vec![]));
            self.stack.push(Value::Function(vec![], vec![]));
        } else {
            let mut context = ScopeContext::default();

            mem::swap(&mut context, &mut self.context);
        }

        Ok(())
    }

    fn simulate_equality(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
        let mut condition = None;
        let mut value = self.pop()?;

        if value == Value::Boolean {
            condition = Some(value);
            value = self.pop()?;
        }

        let assignee = self.pop()?;

        match assignee {
            Value::Uninitialized(index) => {
                let depth = self.contexts.len();

                self.context
                    .local_mut(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, value.clone());

                self.context.add_operation(Operation::DefineLocal);
                self.stack.push(value);
            }
            Value::Any => self.stack.push(Value::Any),
            _ if condition.is_none() => self.stack.push(Value::Boolean),
            _ => {
                self.report_error(ErrorKind::ConditionWithoutAssignment);
                self.stack.push(Value::Any);
            }
        }

        Ok(())
    }

    fn simulate_call(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
        let arguments = self.pop()?;
        let callee = self.pop()?;

        match callee {
            Value::Uninitialized(index) => {
                // TODO create undefined function type and leave local undefined.
            }
            Value::Closure(Some(index)) => {
                let function = self
                    .functions
                    .get(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchFunction(index)))?;

                let arguments_group = arguments;
                let arguments = match &arguments_group {
                    Value::Group(parts) if parts.len() == 1 => match parts.as_slice() {
                        [Value::Group(inner)] => inner.as_slice(),
                        _ => parts.as_slice(),
                    },
                    Value::Group(parts) => parts.as_slice(),
                    _ => &[],
                };

                if function.parameters() == arguments {
                    self.context.add_operation(Operation::Call(index as u8));
                    self.stack.extend_from_slice(function.results());
                } else {
                    self.report_error(ErrorKind::InvalidArguments(
                        function.parameters().to_vec(),
                        arguments.to_vec(),
                    ));
                    self.stack.push(Value::Any);
                }
            }
            _ => {
                self.report_error(ErrorKind::NotCallable(callee));
                self.stack.push(Value::Any);
            }
        }

        Ok(())
    }

    fn simulate_condition(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
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
            (Value::Any, Value::Number(_))
            | (Value::Any, Value::Any)
            | (Value::Number(_), Value::Any) => self.stack.push(Value::Any),
            (lhs, rhs) => {
                self.report_error(ErrorKind::OperandsMustBeNumbers(lhs, rhs));
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
            (Value::Any, Value::Number(_))
            | (Value::Any, Value::Any)
            | (Value::Number(_), Value::Any) => self.stack.push(Value::Any),
            (lhs, rhs) => {
                self.report_error(ErrorKind::OperandsMustBeNumbers(lhs, rhs));
                self.stack.push(Value::Any);
            }
        };

        Ok(())
    }

    fn simulate_grouping(&mut self, node: Node<'a, 'b>) -> TranslationResult<()> {
        let length = node.children();

        if length > u8::MAX as usize {
            self.report_error(ErrorKind::GroupTooLarge(length));
        }

        if length < 1 {
            Err(ErrorKind::EmptyGroup.into())
        } else if length == 1 {
            Ok(())
        } else {
            let mut parts = vec![];

            for _ in 0..length {
                if let Some(part) = self.stack.pop() {
                    parts.push(part);
                }
            }

            if parts.len() == length {
                self.context.add_operation(Operation::Group(length as u8));
                self.stack.push(Value::Group(parts));

                Ok(())
            } else {
                Err(ErrorKind::InvalidGroupSize(length, parts.len()).into())
            }
        }
    }

    fn get_number(&mut self, index: usize) -> TranslationResult<Number> {
        self.numbers
            .get(index)
            .copied()
            .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchNumber(index)))
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
                    self.report_error(ErrorKind::TooManyLocals(index));
                }

                self.stack.push(Value::Uninitialized(index));
                Ok(())
            }
        }
    }

    fn simulate_uri(&mut self, uri: &Uri<'a>) -> TranslationResult<()> {
        let constant = match Text::try_from(*uri) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Text::default()
            }
        };
        let index = self.texts.insert(*uri, constant);

        if index >= u8::MAX as usize {
            self.report_error(ErrorKind::TooManyUris(index));
        }

        self.context
            .add_operation(Operation::ConstantText(index as u8));
        self.stack.push(Value::Text(Some(index)));

        Ok(())
    }

    fn simulate_number(&mut self, number: &grammar::Number<'a>) -> TranslationResult<()> {
        let constant = match Number::try_from(*number) {
            Ok(c) => c,
            Err(e) => {
                self.report_error(e);
                Number::default()
            }
        };
        let index = self.numbers.insert(*number, constant);

        if index >= u8::MAX as usize {
            self.report_error(ErrorKind::TooManyNumbers(index));
        }

        self.context
            .add_operation(Operation::ConstantNumber(index as u8));
        self.stack.push(Value::Number(Some(index)));

        Ok(())
    }

    fn pop(&mut self) -> TranslationResult<Value> {
        self.stack
            .pop()
            .ok_or_else(|| TranslationError::from(ErrorKind::EmptyStack))
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
    use tortuga_executable::ToCode;

    #[test]
    fn undefined_variable() {
        assert_eq!(Translation::try_from("x + 40").unwrap_err().len(), 1);
    }

    #[test]
    fn add_numbers() {
        let executable: Executable = Translation::try_from("(2 + 40)").unwrap().into();
        let code = vec![
            Operation::ConstantNumber(0),
            Operation::ConstantNumber(1),
            Operation::Add,
        ]
        .to_code();

        assert_eq!(
            executable.function(0).unwrap().code().as_slice(),
            code.as_slice()
        );
    }

    #[test]
    fn add_with_variable() {
        let executable: Executable = Translation::try_from("x = 2\nx + 40").unwrap().into();
        let code = vec![
            Operation::ConstantNumber(0),
            Operation::DefineLocal,
            Operation::GetLocal(1),
            Operation::ConstantNumber(1),
            Operation::Add,
        ]
        .to_code();
        assert_eq!(
            executable.function(0).unwrap().code().as_slice(),
            code.as_slice()
        );
        assert_eq!(
            executable.function(0),
            Some(&Function::new(0, 1, code, vec![]))
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
    fn simple() {
        assert_eq!(
            Translation::try_from(include_str!("../../../examples/simple.ta"))
                .unwrap_err()
                .len(),
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
