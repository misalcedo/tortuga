use crate::Program;
use crate::{grammar, CompilationError, ErrorReporter};
use std::mem;
use std::slice::Iter;
use tortuga_executable::{Executable, Function, Number, Operation, Text};

mod capture;
mod error;
mod function;
mod indices;
mod local;
mod number;
mod scope;
mod uri;
mod value;

use crate::grammar::{
    ExpressionKind, ExpressionReference, Identifier, Node, ReferenceIterator, Uri,
};
use crate::translate::capture::Capture;
use crate::translate::error::ErrorKind;
use crate::translate::local::Local;
pub use error::TranslationError;
use function::TypedFunction;
use indices::IndexedSet;
use scope::Scope;
use value::Value;

#[derive(Clone)]
pub struct Translator<'a, Reporter> {
    reporter: Reporter,
    scope: Scope<'a>,
    scopes: Vec<Scope<'a>>,
    functions: Vec<TypedFunction<'a>>,
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
            scope: Default::default(),
            scopes: Default::default(),
            functions: vec![TypedFunction::default()],
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
            let functions: Vec<Function> = self
                .functions
                .into_iter()
                .map(Function::try_from)
                .filter_map(|r| match r {
                    Ok(f) => Some(f),
                    Err(e) => {
                        self.reporter.report_translation_error(e);
                        None
                    }
                })
                .collect();
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
        let scope = mem::replace(&mut self.scope, Scope::default());

        if !self.scopes.is_empty() {
            self.report_error(ErrorKind::BlockNotTerminated);
        }

        let index = scope.function();
        let function = self
            .functions
            .get_mut(index)
            .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchFunction(index)))?;

        if function.initialize(scope).is_some() {
            Err(ErrorKind::FunctionAlreadyInitialized(index).into())
        } else {
            Ok(Value::None)
        }
    }

    fn simulate_expression(&mut self, node: Node<'a, 'b>) -> TranslationResult<Value> {
        match node.expression().kind() {
            ExpressionKind::Number(number) => self.simulate_number(number),
            ExpressionKind::Identifier(identifier) => self.simulate_identifier(identifier),
            ExpressionKind::Uri(uri) => self.simulate_uri(uri),
            ExpressionKind::Equality => self.simulate_equality(node),
            ExpressionKind::Modulo => self.simulate_binary(node, Operation::Remainder),
            ExpressionKind::Subtract => self.simulate_binary(node, Operation::Subtract),
            ExpressionKind::Add => self.simulate_binary(node, Operation::Add),
            ExpressionKind::Divide => self.simulate_binary(node, Operation::Divide),
            ExpressionKind::Multiply => self.simulate_binary(node, Operation::Multiply),
            ExpressionKind::Power => self.simulate_binary(node, Operation::Power),
            ExpressionKind::Call => self.simulate_call(node),
            ExpressionKind::Grouping => self.simulate_grouping(node),
            ExpressionKind::Inequality => self.simulate_binary(node, Operation::NotEqual),
            ExpressionKind::LessThan => self.simulate_binary(node, Operation::Less),
            ExpressionKind::GreaterThan => self.simulate_binary(node, Operation::Greater),
            ExpressionKind::LessThanOrEqualTo => self.simulate_binary(node, Operation::LessOrEqual),
            ExpressionKind::GreaterThanOrEqualTo => {
                self.simulate_binary(node, Operation::GreaterOrEqual)
            }
            ExpressionKind::Block => {
                self.report_error(ErrorKind::BlockOutsideFunction);
                Ok(Value::Any)
            }
            ExpressionKind::Condition => {
                self.report_error(ErrorKind::ConditionOutsideFunction);
                Ok(Value::Any)
            }
        }
    }

    fn simulate_equality(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Equality)?;

        let mut children = node.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, length));
        }

        let lhs = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let rhs = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 1)))?;

        match lhs.expression().kind() {
            ExpressionKind::Call => self.simulate_function_declaration(lhs, rhs),
            ExpressionKind::Identifier(identifier) => {
                self.simulate_variable_assignment(identifier, rhs)
            }
            _ => {
                self.simulate_expression(lhs)?;
                self.simulate_expression(rhs)?;

                self.scope.push_operation(Operation::Equal);

                Ok(Value::Boolean)
            }
        }
    }

    fn simulate_function_declaration(
        &mut self,
        call: Node<'a, 'b>,
        rhs: Node<'a, 'b>,
    ) -> SimulationResult {
        self.assert_kind(&call, ExpressionKind::Call)?;

        let mut children = call.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=3, length));
        }

        let callee = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=3, 0)))?;
        let callee = self.simulate_expression(callee)?;

        match callee {
            Value::Uninitialized(index) => {
                let parameters = children
                    .next()
                    .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=3, 1)))?;
                let function = self.functions.len();
                let depth = self.scopes.len();
                let value = Value::Closure(function);

                if function > u8::MAX as usize {
                    self.report_error(ErrorKind::TooManyFunctions(function));
                }

                self.scope
                    .local_mut(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, value.clone());

                let scope = self.scope.new(function);

                self.scopes.push(mem::replace(&mut self.scope, scope));

                let parameters = self.simulate_parameters(function, parameters)?;

                let condition = children.next();
                let rhs = self.simulate_block(rhs)?;

                self.functions.push(TypedFunction::new(parameters, rhs));

                let enclosing = self
                    .scopes
                    .pop()
                    .ok_or_else(|| TranslationError::from(ErrorKind::EmptyScopes))?;
                let scope = mem::replace(&mut self.scope, enclosing);
                let captures = scope.capture_offsets().map(|offset| offset as u8).collect();

                self.scope
                    .push_operation(Operation::Closure(function as u8, captures));
                self.scope.push_operation(Operation::DefineLocal);

                self.functions
                    .get_mut(scope.function())
                    .ok_or_else(|| {
                        TranslationError::from(ErrorKind::NoSuchFunction(scope.function()))
                    })?
                    .initialize(scope);

                Ok(value)
            }
            _ => self.simulate_call_closure(&mut children, callee),
        }
    }

    fn simulate_block(&mut self, block: Node<'a, 'b>) -> SimulationResult {
        match block.expression().kind() {
            ExpressionKind::Block => {
                let children = block.children();
                let mut result = Value::None;

                if children.len() == 0 {
                    self.report_error(ErrorKind::EmptyBlock);
                }

                for child in children {
                    result = self.simulate_expression(child)?;
                }

                Ok(result)
            }
            _ => self.simulate_expression(block),
        }
    }

    fn simulate_variable_assignment(
        &mut self,
        identifier: &Identifier<'a>,
        rhs: Node<'a, 'b>,
    ) -> SimulationResult {
        let lhs = self.simulate_identifier(identifier)?;
        let rhs = self.simulate_expression(rhs)?;

        match lhs {
            Value::Uninitialized(index) => {
                let depth = self.scopes.len();

                self.scope
                    .local_mut(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, rhs.clone());

                self.scope.push_operation(Operation::DefineLocal);

                Ok(rhs)
            }
            _ => {
                self.scope.push_operation(Operation::Equal);

                Ok(Value::Boolean)
            }
        }
    }

    fn simulate_call(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Call)?;

        let mut children = node.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, length));
        }

        let callee = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let callee = self.simulate_expression(callee)?;

        self.simulate_call_closure(&mut children, callee)
    }

    fn simulate_call_closure(
        &mut self,
        children: &mut ReferenceIterator<'a, 'b, Iter<'b, ExpressionReference>>,
        callee: Value,
    ) -> SimulationResult {
        let arguments = children
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::MissingChildren(2..=2, 1)))?;
        let arguments = self.simulate_grouping(arguments)?;

        match callee {
            Value::Closure(index) => {
                let function = self
                    .functions
                    .get(index)
                    .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchFunction(index)))?;
                let parameters = function.parameters();

                if parameters == &arguments {
                    if parameters.len() != arguments.len() {
                        self.scope.push_operation(Operation::Separate);
                    }

                    self.scope
                        .push_operation(Operation::Call(parameters.len() as u8));

                    Ok(function.results().clone())
                } else {
                    self.report_error(ErrorKind::InvalidArguments(parameters.clone(), arguments));
                    Ok(Value::Any)
                }
            }
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
            self.scope.push_operation(operation);
            Ok(value)
        } else {
            self.report_error(ErrorKind::OperandsMustBeNumbers(lhs, rhs));
            Ok(Value::Any)
        }
    }

    fn simulate_parameters(&mut self, function: usize, node: Node<'a, 'b>) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Grouping)?;

        let children = node.children();
        let length = children.len();

        if length > u8::MAX as usize {
            self.report_error(ErrorKind::GroupTooLarge(length));
        }

        if length < 1 {
            self.report_error(ErrorKind::EmptyGroup);
            Ok(Value::Any)
        } else {
            let mut parts = vec![];
            let depth = self.scopes.len();

            for (index, child) in children.enumerate() {
                let parameter = match child.expression().kind() {
                    ExpressionKind::Identifier(identifier) => {
                        self.simulate_identifier(identifier)?
                    }
                    kind => {
                        self.report_error(ErrorKind::ExpectedKind(
                            "Identifier".to_string(),
                            kind.to_string(),
                        ));
                        Value::Any
                    }
                };

                match parameter {
                    Value::Uninitialized(index) => {
                        self.scope
                            .local_mut(index)
                            .ok_or_else(|| TranslationError::from(ErrorKind::NoSuchLocal(index)))?
                            .initialize(depth, Value::Any);
                    }
                    _ => self.report_error(ErrorKind::LocalInFunctionSignature(function, index)),
                }

                parts.push(Value::Any);
            }

            Ok(Value::group(parts))
        }
    }

    fn simulate_grouping(&mut self, node: Node<'a, 'b>) -> SimulationResult {
        self.assert_kind(&node, ExpressionKind::Grouping)?;

        let children = node.children();
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

            if parts.len() > 1 {
                self.scope.push_operation(Operation::Group(length as u8));
            }

            Ok(Value::group(parts))
        }
    }

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) -> SimulationResult {
        match self.resolve_local(identifier) {
            Some(local) => {
                self.scope
                    .push_operation(Operation::GetLocal(local.offset() as u8));

                Ok(local.into())
            }
            None => match self.resolve_capture(identifier)? {
                Some(capture) => {
                    self.scope
                        .push_operation(Operation::GetCapture(capture.offset() as u8));
                    Ok(capture.kind().clone())
                }
                None => {
                    let index = self.scope.push_local(*identifier);

                    if index >= u8::MAX as usize {
                        self.report_error(ErrorKind::TooManyLocals(index));
                    }

                    Ok(Value::Uninitialized(index))
                }
            },
        }
    }

    fn simulate_uri(&mut self, uri: &Uri<'a>) -> SimulationResult {
        let constant = Text::from(*uri);
        let index = self.texts.insert(*uri, constant);

        if index > u8::MAX as usize {
            self.report_error(ErrorKind::TooManyUris(index));
        }

        self.scope
            .push_operation(Operation::ConstantText(index as u8));

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

        if index > u8::MAX as usize {
            self.report_error(ErrorKind::TooManyNumbers(index));
        }

        self.scope
            .push_operation(Operation::ConstantNumber(index as u8));

        Ok(Value::Number(Some(index)))
    }

    fn resolve_capture(&mut self, name: &Identifier<'a>) -> TranslationResult<Option<Capture>> {
        let mut iterator = self.scopes.iter_mut().chain(Some(&mut self.scope));
        let mut capture = None;

        while let Some(enclosing) = iterator.next() {
            if let Some(local) = enclosing.resolve_local(name) {
                enclosing.capture_local(&local);

                capture = Some((local.offset(), local.kind().clone()));

                break;
            }
        }

        let (mut offset, kind) = match capture {
            None => return Ok(None),
            Some(p) => p,
        };
        let scope = iterator
            .next()
            .ok_or_else(|| TranslationError::from(ErrorKind::EmptyScopes))?;

        offset = scope.push_capture(offset, true, kind.clone());

        if scope.captures() > u8::MAX as usize {
            self.reporter
                .report_translation_error(ErrorKind::TooManyCaptures(scope.captures()).into());
        }

        while let Some(scope) = iterator.next() {
            offset = scope.push_capture(offset, false, kind.clone());

            if scope.captures() > u8::MAX as usize {
                self.reporter
                    .report_translation_error(ErrorKind::TooManyCaptures(scope.captures()).into());
            }
        }

        Ok(self.scope.capture(offset))
    }

    fn resolve_local(&mut self, name: &Identifier<'a>) -> Option<Local<'a>> {
        let local = self.scope.resolve_local(name)?;

        if local.depth().is_none() {
            self.report_error(ErrorKind::ReferenceSelfInInitializer(name.to_string()));
        }

        Some(local)
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
        let translator = Translator::new(vec![]);

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
        let executable: Executable =
            Translation::try_from(include_str!("../../../examples/simple.ta"))
                .unwrap()
                .into();
        let script = vec![
            Operation::Closure(1, vec![]),
            Operation::DefineLocal,
            Operation::Closure(2, vec![1]),
            Operation::DefineLocal,
            Operation::GetLocal(1),
            Operation::ConstantNumber(0),
            Operation::Call(1),
            Operation::GetLocal(2),
            Operation::ConstantNumber(0),
            Operation::Call(1),
            Operation::Subtract,
            Operation::ConstantNumber(3),
            Operation::Equal,
        ]
        .to_code();

        assert_eq!(
            executable.function(0).unwrap().code().as_slice(),
            script.as_slice()
        );

        let f = vec![
            Operation::GetLocal(1),
            Operation::ConstantNumber(0),
            Operation::Power,
            Operation::ConstantNumber(1),
            Operation::GetLocal(1),
            Operation::Multiply,
            Operation::Add,
            Operation::ConstantNumber(2),
            Operation::ConstantNumber(0),
            Operation::Divide,
            Operation::Subtract,
        ]
        .to_code();
        assert_eq!(
            executable.function(1).unwrap().code().as_slice(),
            f.as_slice()
        );

        let g = vec![
            Operation::ConstantNumber(3),
            Operation::GetCapture(0),
            Operation::GetLocal(1),
            Operation::Call(1),
            Operation::Add,
        ]
        .to_code();
        assert_eq!(
            executable.function(2).unwrap().code().as_slice(),
            g.as_slice()
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
