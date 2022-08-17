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

use crate::grammar::{ExpressionKind, Identifier, Node, Uri};
use crate::translate::error::ErrorKind;
use context::ScopeContext;
pub use error::TranslationError;
use function::TypedFunction;
use indices::IndexedSet;
use value::Value;

pub struct Translator<'a, Reporter> {
    reporter: Reporter,
    program: Program<'a>,
    context: ScopeContext<'a>,
    contexts: Vec<ScopeContext<'a>>,
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
type SimulationResult = Result<Value, TranslationError>;

impl<'a, 'b, R> Translator<'a, R>
where
    'a: 'b,
    R: ErrorReporter,
{
    pub fn new(program: Program<'a>, reporter: R) -> Self {
        Translator {
            reporter,
            program,
            context: Default::default(),
            contexts: Default::default(),
            functions: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
        }
    }

    // TODO:
    // * Prevents blocks outside of assignment.
    // * Prevent local variable in an assignment that is not a block.
    // * Check function type signatures.
    pub fn translate(mut self) -> Result<Translation<'a>, R> {
        if let Err(e) = self.simulate() {
            self.report_error(e);
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            let functions: Vec<Function> = self.functions.into_iter().map(Function::from).collect();
            let executable = Executable::new(functions, self.numbers, self.texts);

            Ok(Translation {
                input: self.program,
                output: executable,
            })
        }
    }

    fn simulate(&mut self) -> SimulationResult {
        let mut iterator = self.program.roots();

        while let Some(root) = iterator.next() {
            self.simulate_expression(root)?;
        }

        self.simulate_program()
    }

    // TODO: Figure out how to denote the number of locals in the script.
    fn simulate_program(&mut self) -> SimulationResult {
        let mut context = ScopeContext::default();

        mem::swap(&mut self.context, &mut context);

        if !self.contexts.is_empty() {
            //TODO: self.report_error(ErrorKind::ExpectedEndOfBlock);
        }

        let function = Function::from(context);
        self.functions
            .push(TypedFunction::new(function, vec![], vec![]));

        Ok(Value::Any)
    }

    fn simulate_expression(&mut self, node: Node<'a, 'b>) -> TranslationResult<Value> {
        match node.expression().kind() {
            ExpressionKind::Block => self.simulate_block(node),
            ExpressionKind::Equality => self.simulate_equality(node),
            ExpressionKind::Modulo => self.simulate_binary(node, Operation::Remainder),
            ExpressionKind::Subtract => self.simulate_binary(node, Operation::Subtract),
            ExpressionKind::Add => self.simulate_binary(node, Operation::Add),
            ExpressionKind::Divide => self.simulate_binary(node, Operation::Divide),
            ExpressionKind::Multiply => self.simulate_binary(node, Operation::Multiply),
            ExpressionKind::Power => self.simulate_binary(node, Operation::Power),
            ExpressionKind::Call => self.simulate_call(node),
            ExpressionKind::Grouping => self.simulate_grouping(node, true),
            ExpressionKind::Condition => self.simulate_condition(node),
            ExpressionKind::Inequality => self.simulate_binary(node, Operation::Equal),
            ExpressionKind::LessThan => self.simulate_binary(node, Operation::Less),
            ExpressionKind::GreaterThan => self.simulate_binary(node, Operation::Greater),
            ExpressionKind::LessThanOrEqualTo => self.simulate_binary(node, Operation::Greater),
            ExpressionKind::GreaterThanOrEqualTo => self.simulate_binary(node, Operation::Less),
            ExpressionKind::Number(number) => self.simulate_number(number),
            ExpressionKind::Identifier(identifier) => self.simulate_identifier(identifier),
            ExpressionKind::Uri(uri) => self.simulate_uri(uri),
        }
    }

    fn simulate_block(&mut self, node: Node<'a, 'b>) -> SimulationResult {
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
            Ok(Value::Function(vec![], vec![]))
        } else {
            let mut context = ScopeContext::default();

            mem::swap(&mut context, &mut self.context);

            Ok(Value::Any)
        }
    }

    fn simulate_equality(&mut self, node: Node<'a, 'b>) -> SimulationResult {
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

        Ok(Value::Any)
    }

    fn simulate_call(&mut self, node: Node<'a, 'b>) -> TranslationResult<Value> {
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

                let kind = function.kind();

                if kind == arguments {
                    if kind.len() != arguments.len() {
                        self.context.add_operation(Operation::Separate);
                    }

                    self.context.add_operation(Operation::Call(index as u8));
                    self.stack.extend_from_slice(function.results());
                } else {
                    self.report_error(ErrorKind::InvalidArguments(
                        function.parameters().to_vec(),
                        vec![arguments],
                    ));
                    self.stack.push(Value::Any);
                }
            }
            _ => {
                self.report_error(ErrorKind::NotCallable(callee));
                self.stack.push(Value::Any);
            }
        }

        Ok(Value::Any)
    }

    fn simulate_condition(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        Ok(Value::Any)
    }

    fn simulate_binary(&mut self, node: Node<'a, 'b>, operation: Operation) -> SimulationResult {
        let children = node.children();
        let length = children.length();
        let iterator = children.into_iter();

        let lhs = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2, length)))?;
        let lhs = self.simulate_expression(lhs)?;

        length += 1;

        let rhs = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2, length)))?;
        let rhs = self.simulate_expression(rhs)?;

        length += 1;
        length += children.count();

        if length != 2 {
            self.report_error(ErrorKind::TooManyChildren(2, length));
        }

        let value = Value::Number(None);

        if lhs == value && rhs == value {
            self.context.add_operation(operation);
            Ok(value)
        } else {
            self.report_error(ErrorKind::OperandsMustBeNumbers(lhs, rhs));
            Ok(Value::Any)
        }
    }

    fn simulate_grouping(&mut self, node: Node<'a, 'b>, emit_operation: bool) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Grouping)?;

        let children = node.children();
        let length = children.len();

        if length > u8::MAX as usize {
            self.report_error(ErrorKind::GroupTooLarge(length));
        }

        if length < 1 {
            Err(ErrorKind::EmptyGroup.into())
        } else if length == 1 {
            let child = children
                .into_iter()
                .next()
                .ok_or_else(|| TranslationError::from(ErrorKind::EmptyGroup))?;

            self.simulate_expression(child)
        } else {
            let mut parts = vec![];

            for child in children {
                parts.push(self.simulate_expression(child)?);
            }

            if emit_operation {
                self.context.add_operation(Operation::Group(length as u8));
            }

            Ok(Value::Group(parts))
        }
    }

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) -> SimulationResult {
        match self.context.resolve_local(identifier) {
            Some(local) => {
                self.context
                    .add_operation(Operation::GetLocal(local.offset() as u8));

                Ok(local.kind().clone())
            }
            None => {
                let index = self.context.add_local(*identifier);

                if index >= u8::MAX as usize {
                    self.report_error(ErrorKind::TooManyLocals(index));
                }

                Ok(Value::Uninitialized(index))
            }
        }
    }

    fn simulate_uri(&mut self, uri: &Uri<'a>) -> SimulationResult {
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

        Ok(Value::Text(Some(index)))
    }

    fn simulate_number(&mut self, number: &grammar::Number<'a>) -> SimulationResult {
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

        Ok(Value::Number(Some(index)))
    }

    fn assert_kind(&mut self, node: &Node, expected: ExpressionKind) -> TranslationResult<()> {
        let actual = node.expression().kind();

        if actual == &expected {
            Ok(())
        } else {
            Err(ErrorKind::ExpectedKind(expected.to_string(), actual.to_string()).into())
        }
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

        Translator::new(program, Vec::default()).translate()
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
