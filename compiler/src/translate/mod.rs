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

#[derive(Clone)]
pub struct Translator<'a, Reporter> {
    reporter: Reporter,
    context: ScopeContext<'a>,
    contexts: Vec<ScopeContext<'a>>,
    functions: Vec<TypedFunction>,
    numbers: IndexedSet<grammar::Number<'a>, Number>,
    texts: IndexedSet<Uri<'a>, Text>,
}

#[derive(Clone, Debug, Default, PartialEq)]
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
    pub fn new(reporter: R) -> Self {
        Translator {
            reporter,
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
    pub fn translate(mut self, program: Program<'a>) -> Result<Translation<'a>, R> {
        if let Err(e) = self.simulate(&program) {
            self.report_error(e);
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            let functions: Vec<Function> = self.functions.into_iter().map(Function::from).collect();
            let executable = Executable::new(functions, self.numbers, self.texts);

            Ok(Translation {
                input: program,
                output: executable,
            })
        }
    }

    fn simulate(&mut self, program: &Program<'a>) -> SimulationResult {
        let mut iterator = program.roots();

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
            .push(TypedFunction::new(function, Value::None, Value::None));

        Ok(Value::Any)
    }

    fn simulate_expression(&mut self, node: Node<'a, 'b>) -> TranslationResult<Value> {
        match node.expression().kind() {
            ExpressionKind::Block => {
                self.report_error(ErrorKind::BlockOutsideFunction);
                Ok(Value::Any)
            }
            ExpressionKind::Equality => self.simulate_equality(node),
            ExpressionKind::Modulo => self.simulate_binary(node, Operation::Remainder),
            ExpressionKind::Subtract => self.simulate_binary(node, Operation::Subtract),
            ExpressionKind::Add => self.simulate_binary(node, Operation::Add),
            ExpressionKind::Divide => self.simulate_binary(node, Operation::Divide),
            ExpressionKind::Multiply => self.simulate_binary(node, Operation::Multiply),
            ExpressionKind::Power => self.simulate_binary(node, Operation::Power),
            ExpressionKind::Call => self.simulate_call(node),
            ExpressionKind::Grouping => self.simulate_grouping(node, true),
            ExpressionKind::Condition => {
                self.report_error(ErrorKind::ConditionOutsideFunction);
                Ok(Value::Any)
            }
            ExpressionKind::Inequality => {
                self.report_error(ErrorKind::ComparisonOutsideCondition(Operation::NotEqual));
                Ok(Value::Any)
            }
            ExpressionKind::LessThan => {
                self.report_error(ErrorKind::ComparisonOutsideCondition(Operation::Less));
                Ok(Value::Any)
            }
            ExpressionKind::GreaterThan => {
                self.report_error(ErrorKind::ComparisonOutsideCondition(Operation::Greater));
                Ok(Value::Any)
            }
            ExpressionKind::LessThanOrEqualTo => {
                self.report_error(ErrorKind::ComparisonOutsideCondition(
                    Operation::LessOrEqual,
                ));
                Ok(Value::Any)
            }
            ExpressionKind::GreaterThanOrEqualTo => {
                self.report_error(ErrorKind::ComparisonOutsideCondition(
                    Operation::GreaterOrEqual,
                ));
                Ok(Value::Any)
            }
            ExpressionKind::Number(number) => self.simulate_number(number),
            ExpressionKind::Identifier(identifier) => self.simulate_identifier(identifier),
            ExpressionKind::Uri(uri) => self.simulate_uri(uri),
        }
    }

    fn simulate_equality(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Equality)?;

        let mut children = node.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, length));
        }

        let assignee = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let assignee = self.simulate_expression(assignee)?;

        match assignee {
            Value::Uninitialized(index) => {
                let value = children
                    .next()
                    .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 1)))?;
                let value = self.simulate_expression(value)?;
                let depth = self.contexts.len();

                self.context
                    .local_mut(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, value.clone());

                self.context.add_operation(Operation::DefineLocal);

                Ok(value)
            }
            Value::UninitializedFunction(index, parameters) => {
                let depth = self.contexts.len();

                self.context
                    .local_mut(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, value.clone());

                self.context.add_operation(Operation::DefineLocal);

                Ok(value)
            }
            Value::Any => Ok(Value::Any),
            _ => {
                self.report_error(ErrorKind::NotAssignable(assignee));
                Ok(Value::Any)
            }
        }
    }

    fn simulate_block(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        self.simulate_expression(node)
    }

    fn simulate_call(&mut self, node: Node<'a, 'b>) -> TranslationResult<Value> {
        self.assert_kind(&node, ExpressionKind::Call)?;

        let mut children = node.children();
        let length = children.len();

        if length > 3 {
            self.report_error(ErrorKind::TooManyChildren(2..=3, length));
        }

        let callee = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=3, 0)))?;
        let callee = self.simulate_expression(callee)?;

        let arguments = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=3, 1)))?;
        let arguments = self.simulate_grouping(arguments, false)?;

        let _condition = match children.next() {
            None => Value::None,
            Some(condition) => self.simulate_condition(condition)?,
        };

        match callee {
            Value::Uninitialized(index) => Ok(Value::uninitialized_function(index, arguments)),
            Value::Closure(index) => {
                let function = self
                    .functions
                    .get(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchFunction(index)))?;

                let parameters = function.parameters();

                if parameters == &arguments {
                    if parameters.len() != arguments.len() {
                        self.context.add_operation(Operation::Separate);
                    }

                    self.context.add_operation(Operation::Call(index as u8));

                    Ok(function.results().clone())
                } else {
                    self.report_error(ErrorKind::InvalidArguments(parameters.clone(), arguments));
                    Ok(Value::Any)
                }
            }
            Value::Any => Ok(Value::Any),
            _ => {
                self.report_error(ErrorKind::NotCallable(callee));
                Ok(Value::Any)
            }
        }
    }

    fn simulate_condition(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Condition)?;

        match node.expression().kind() {
            ExpressionKind::Equality => {
                self.simulate_binary(node, Operation::Equal)?;
                Ok(Value::Boolean)
            }
            ExpressionKind::Inequality => {
                self.simulate_binary(node, Operation::NotEqual)?;
                Ok(Value::Boolean)
            }
            ExpressionKind::LessThan => {
                self.simulate_binary(node, Operation::Less)?;
                Ok(Value::Boolean)
            }
            ExpressionKind::GreaterThan => {
                self.simulate_binary(node, Operation::Greater)?;
                Ok(Value::Boolean)
            }
            ExpressionKind::LessThanOrEqualTo => {
                self.simulate_binary(node, Operation::LessOrEqual)?;
                Ok(Value::Boolean)
            }
            ExpressionKind::GreaterThanOrEqualTo => {
                self.simulate_binary(node, Operation::GreaterOrEqual)?;
                Ok(Value::Boolean)
            }
            kind => {
                self.report_error(ErrorKind::InvalidCondition(kind.to_string()));
                Ok(Value::Any)
            }
        }
    }

    fn simulate_binary(&mut self, node: Node<'a, 'b>, operation: Operation) -> SimulationResult {
        let mut children = node.children();

        if children.len() != 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, children.len()));
        }

        let lhs = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let lhs = self.simulate_expression(lhs)?;

        let rhs = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 1)))?;
        let rhs = self.simulate_expression(rhs)?;

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

        let mut children = node.children();
        let length = children.len();

        if length > u8::MAX as usize {
            self.report_error(ErrorKind::GroupTooLarge(length));
        }

        if length < 1 {
            self.report_error(ErrorKind::EmptyGroup);
            Ok(Value::Any)
        } else {
            let mut parts = vec![];

            for child in children {
                parts.push(self.simulate_expression(child)?);
            }

            if emit_operation && parts.len() > 1 {
                self.context.add_operation(Operation::Group(length as u8));
            }

            Ok(Value::group(parts))
        }
    }

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) -> SimulationResult {
        match self.context.resolve_local(identifier) {
            Some(local) => {
                self.context
                    .add_operation(Operation::GetLocal(local.offset() as u8));

                Ok(local.into())
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
        let constant = Text::from(*uri);
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
        let mut translator = Translator::new(vec![]);

        translator.translate(program)
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
        let executable: Executable = Translation::try_from("(4 + 40) - 2").unwrap().into();
        let code = vec![
            Operation::ConstantNumber(0),
            Operation::ConstantNumber(1),
            Operation::Add,
            Operation::ConstantNumber(2),
            Operation::Subtract,
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
