use crate::compiler::{CompilationError, ErrorReporter};
use crate::{Executable, Function, Number, Operation, Program, Text};
use std::borrow::Cow;
use std::mem;
use std::slice::Iter;
use std::str::FromStr;

mod capture;
mod error;
mod function;
mod local;
mod scope;
mod value;

use crate::collections::{IndexedSet, NonEmptyStack};
use crate::compiler::analysis::capture::Capture;
use crate::compiler::analysis::error::ErrorKind;
use crate::compiler::analysis::local::Local;
use crate::compiler::grammar;
use crate::compiler::grammar::{
    ExpressionKind, ExpressionReference, Identifier, Node, ReferenceIterator, Uri,
};
pub use error::AnalysisError;
use function::TypedFunction;
pub use scope::Scope;
pub use value::Value;

type AnalysisResult = Result<Value, AnalysisError>;
type Excerpt<'a> = Cow<'a, str>;

/// Analyze a [`Program`] to:
/// * Ensure type safety.
/// * Disambiguate assignment from equality.
/// * Map identifiers to local offsets.
/// * Identify captured locals.
/// * Find unreachable code.
/// * Find dead code.
/// * Find unused locals.
#[derive(Clone)]
pub struct SemanticAnalyzer<'a, Reporter> {
    reporter: Reporter,
    scopes: NonEmptyStack<Scope<'a>>,
    functions: Vec<TypedFunction<'a>>,
    numbers: IndexedSet<grammar::Number<'a>, Number>,
    texts: IndexedSet<Uri<'a>, Text>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Analysis<'a> {
    input: Program<'a>,
    scopes: NonEmptyStack<Scope<'a>>,
    numbers: IndexedSet<grammar::Number<'a>, Number>,
    texts: IndexedSet<Uri<'a>, Text>,
}

impl<'a> Analysis<'a> {
    pub fn new<E>(program: Program<'a>, analyzer: SemanticAnalyzer<'a, E>) -> Self {
        Analysis {
            input: program,
            scopes: analyzer.scopes,
            numbers: analyzer.numbers,
            texts: analyzer.texts,
        }
    }
}

impl<'a, 'b, R> SemanticAnalyzer<'a, R>
where
    'a: 'b,
    R: ErrorReporter,
{
    pub fn new(reporter: R) -> Self {
        SemanticAnalyzer {
            reporter,
            scopes: Default::default(),
            functions: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
        }
    }

    // TODO:
    // * Prevent local variable in an assignment that is not a block.
    pub fn analyze(mut self, program: Program<'a>) -> Result<Analysis<'a>, R> {
        if let Err(e) = self.analyze_program(&program) {
            self.report_error(e);
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(Analysis::new(program, self))
        }
    }

    fn analyze_program(&mut self, program: &Program<'a>) -> AnalysisResult {
        let mut iterator = program.roots().peekable();
        let mut result = Value::None;

        self.functions.push(TypedFunction::default());

        while let Some(root) = iterator.next() {
            if iterator.peek().is_some() {
                self.simulate_statement(root)?;
            } else {
                result = self.simulate_expression(root)?;
            }
        }

        self.scope.push_operation(Operation::Return);

        let scope = mem::replace(&mut self.scope, Scope::default());

        if !self.scopes.is_empty() {
            self.report_error(ErrorKind::BlockNotTerminated);
        }

        self.initialize_function(scope)?;

        Ok(result)
    }

    fn simulate_statement(&mut self, node: Node<'a, 'b>) -> AnalysisResult {
        let value = self.simulate_expression(node)?;

        if Value::None != value {
            self.scope.push_operation(Operation::Pop);
        }

        Ok(value)
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
            ExpressionKind::Grouping => self.simulate_grouping(node, None),
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

    fn simulate_equality(&mut self, node: Node<'a, 'b>) -> AnalysisResult {
        self.assert_kind(&node, ExpressionKind::Equality)?;

        let mut children = node.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, length));
        }

        let lhs = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let rhs = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=2, 1)))?;

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
    ) -> AnalysisResult {
        self.assert_kind(&call, ExpressionKind::Call)?;

        let mut children = call.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=3, length));
        }

        let callee = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=3, 0)))?;
        let callee = self.simulate_expression(callee)?;

        match callee {
            Value::Uninitialized(index) => {
                let parameters = children
                    .next()
                    .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=3, 1)))?;
                let function = self.functions.len();
                let depth = self.scopes.len();
                let value = Value::Closure(function);

                if function > u8::MAX as usize {
                    self.report_error(ErrorKind::TooManyFunctions(function));
                }

                let offset = self
                    .scope
                    .local_mut(index)
                    .ok_or_else(|| AnalysisError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, value.clone());

                let scope = self.scope.new(function);

                self.scopes.push(mem::replace(&mut self.scope, scope));

                let parameters = self.simulate_parameters(function, parameters)?;

                self.scope.set_arity(parameters.iter().count());

                let condition = children.next();
                let rhs = self.simulate_block(rhs)?;

                self.functions.push(TypedFunction::new(parameters, rhs));

                let enclosing = self
                    .scopes
                    .pop()
                    .ok_or_else(|| AnalysisError::from(ErrorKind::EmptyScopes))?;
                let scope = mem::replace(&mut self.scope, enclosing);
                let captures = scope.capture_offsets().map(|offset| offset as u8).collect();

                self.scope
                    .push_operation(Operation::Closure(function as u8, captures));
                self.scope.push_operation(Operation::DefineLocal);
                self.scope.push_operation(Operation::GetLocal(offset as u8));

                self.initialize_function(scope)?;

                Ok(value)
            }
            _ => self.simulate_call_closure(&mut children, callee),
        }
    }

    fn initialize_function(&mut self, scope: Scope<'a>) -> TranslationResult<()> {
        let index = scope.function();
        let function = self
            .functions
            .get_mut(index)
            .ok_or_else(|| AnalysisError::from(ErrorKind::NoSuchFunction(index)))?;

        if function.initialize(scope).is_some() {
            self.report_error(ErrorKind::FunctionAlreadyInitialized(index));
        }

        Ok(())
    }

    fn simulate_block(&mut self, block: Node<'a, 'b>) -> AnalysisResult {
        let result = match block.expression().kind() {
            ExpressionKind::Block => {
                let mut children = block.children().peekable();
                let mut result = Value::None;

                if children.len() == 0 {
                    self.report_error(ErrorKind::EmptyBlock);
                }

                while let Some(child) = children.next() {
                    if children.peek().is_some() {
                        result = self.simulate_statement(child)?;
                    } else {
                        result = self.simulate_expression(child)?;
                    }
                }

                Ok(result)
            }
            _ => self.simulate_expression(block),
        };

        self.scope.push_operation(Operation::Return);

        result
    }

    fn simulate_variable_assignment(
        &mut self,
        identifier: &Identifier<'a>,
        rhs: Node<'a, 'b>,
    ) -> AnalysisResult {
        let lhs = self.simulate_identifier(identifier)?;
        let rhs = self.simulate_expression(rhs)?;

        match lhs {
            Value::Uninitialized(index) => {
                let depth = self.scopes.len();
                let offset = self
                    .scope
                    .local_mut(index)
                    .ok_or_else(|| AnalysisError::from(ErrorKind::NoSuchLocal(index)))?
                    .initialize(depth, rhs.clone());

                self.scope.push_operation(Operation::DefineLocal);
                self.scope.push_operation(Operation::GetLocal(offset as u8));

                Ok(rhs)
            }
            _ => {
                self.scope.push_operation(Operation::Equal);

                Ok(Value::Boolean)
            }
        }
    }

    fn simulate_call(&mut self, node: Node<'a, 'b>) -> AnalysisResult {
        self.assert_kind(&node, ExpressionKind::Call)?;

        let mut children = node.children();
        let length = children.len();

        if length > 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, length));
        }

        let callee = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let callee = self.simulate_expression(callee)?;

        self.simulate_call_closure(&mut children, callee)
    }

    fn simulate_call_closure(
        &mut self,
        children: &mut ReferenceIterator<'a, 'b, Iter<'b, ExpressionReference>>,
        callee: Value,
    ) -> AnalysisResult {
        let arguments = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=2, 1)))?;

        match callee {
            Value::Closure(index) => {
                let expect = self
                    .functions
                    .get(index)
                    .map(TypedFunction::parameters)
                    .map(Value::len);
                let arguments = self.simulate_grouping(arguments, expect)?;
                let function = self
                    .functions
                    .get(index)
                    .ok_or_else(|| AnalysisError::from(ErrorKind::NoSuchFunction(index)))?;
                let parameters = function.parameters();

                if parameters == &arguments {
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

    fn simulate_condition(&mut self, node: Node<'a, 'b>) -> AnalysisResult {
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

    fn simulate_binary(&mut self, node: Node<'a, 'b>, operation: Operation) -> AnalysisResult {
        let mut children = node.children();

        if children.len() != 2 {
            self.report_error(ErrorKind::TooManyChildren(2..=2, children.len()));
        }

        let lhs = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=2, 0)))?;
        let lhs = self.simulate_expression(lhs)?;

        let rhs = children
            .next()
            .ok_or_else(|| AnalysisError::from(ErrorKind::MissingChildren(2..=2, 1)))?;
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

    fn simulate_parameters(&mut self, function: usize, node: Node<'a, 'b>) -> AnalysisResult {
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
                            .ok_or_else(|| AnalysisError::from(ErrorKind::NoSuchLocal(index)))?
                            .initialize(depth, Value::Any);
                    }
                    _ => self.report_error(ErrorKind::LocalInFunctionSignature(function, index)),
                }

                parts.push(Value::Any);
            }

            Ok(Value::group(parts))
        }
    }

    fn simulate_grouping(&mut self, node: Node<'a, 'b>, expect: Option<usize>) -> AnalysisResult {
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
                let value = match child.expression().kind() {
                    ExpressionKind::Grouping if length == 1 => {
                        self.report_error(ErrorKind::UnnecessaryParenthesis);
                        self.simulate_grouping(child, expect)?
                    }
                    _ => self.simulate_expression(child)?,
                };

                parts.push(value);
            }

            match (expect, parts.as_slice()) {
                (Some(expect), [Value::Group(actual)]) if expect == actual.len() => {
                    self.scope.push_operation(Operation::Separate)
                }
                (None, _) if parts.len() > 1 => {
                    self.scope.push_operation(Operation::Group(length as u8))
                }
                _ => {}
            };

            Ok(Value::group(parts))
        }
    }

    fn simulate_identifier(&mut self, identifier: &Identifier<'a>) -> AnalysisResult {
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

    fn simulate_uri(&mut self, uri: &Uri<'a>) -> AnalysisResult {
        let constant = Text::from(*uri);
        let index = self.texts.insert(*uri, constant);

        if index > u8::MAX as usize {
            self.report_error(ErrorKind::TooManyUris(index));
        }

        self.scope
            .push_operation(Operation::ConstantText(index as u8));

        Ok(Value::Text(Some(index)))
    }

    fn simulate_number(&mut self, number: &grammar::Number<'a>) -> AnalysisResult {
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
            .ok_or_else(|| AnalysisError::from(ErrorKind::EmptyScopes))?;

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

    fn report_error<E: Into<AnalysisError>>(&mut self, error: E) {
        self.reporter.report_translation_error(error.into());
    }
}
