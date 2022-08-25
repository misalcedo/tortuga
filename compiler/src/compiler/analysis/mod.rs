use crate::{grammar, CompilationError, ErrorReporter, Number, Program, Text};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

mod capture;
mod error;
mod function;
mod local;
mod result;
mod types;

use crate::collections::{IndexedSet, NonEmptyStack};
use crate::grammar::{ExpressionKind, ExpressionReference, Identifier, Node, Uri};
pub use capture::Capture;
pub use error::AnalysisError;
use error::ErrorKind;
pub use function::Function;
pub use local::Local;
pub use result::Analysis;
pub use types::Type;

type AnalysisResult = Result<Type, AnalysisError>;
static STATEMENT_KINDS: &[ExpressionKind<'static>] = &[ExpressionKind::Equality];

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
    assignments: HashSet<ExpressionReference>,
    functions: NonEmptyStack<Function<'a>>,
    types: HashMap<ExpressionReference, Type>,
    numbers: IndexedSet<Cow<'a, str>, Number>,
    texts: IndexedSet<Cow<'a, str>, Text>,
}

impl<'a, 'b, R> SemanticAnalyzer<'a, R>
where
    'a: 'b,
    R: ErrorReporter,
{
    pub fn new(reporter: R) -> Self {
        SemanticAnalyzer {
            reporter,
            assignments: Default::default(),
            functions: Default::default(),
            types: Default::default(),
            numbers: Default::default(),
            texts: Default::default(),
        }
    }

    pub fn analyze(mut self, program: Program<'a>) -> Result<Analysis<'a>, R> {
        match self.analyze_program(&program) {
            Ok(kind) => {
                if kind.converts_to(&Type::None) {
                    self.report_error(ErrorKind::EmptyProgram);
                }
            }
            Err(e) => self.report_error(e),
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(Analysis::new(program, self))
        }
    }

    fn analyze_program(&mut self, program: &Program<'a>) -> AnalysisResult {
        let mut iterator = program.roots().peekable();
        let mut result = Type::None;

        while let Some(root) = iterator.next() {
            if iterator.peek().is_some() {
                self.analyze_statement(root)?;
            } else {
                result = self.analyze_expression(root)?;
            }
        }

        Ok(result)
    }

    fn analyze_statement(&mut self, statement: Node<'a, 'b>) -> AnalysisResult {
        if STATEMENT_KINDS.contains(statement.kind()) {
            let reference = statement.reference();

            self.analyze_expression(statement)?;

            if self.assignments.contains(&reference) {
                return Ok(Type::None);
            }
        }

        self.report_error(ErrorKind::UnusedExpression);

        Ok(Type::None)
    }

    fn analyze_expression(&mut self, expression: Node<'a, 'b>) -> AnalysisResult {
        let reference = expression.reference();
        let kind = match expression.kind() {
            ExpressionKind::Block => Type::None,
            ExpressionKind::Equality => Type::None,
            ExpressionKind::Modulo => Type::None,
            ExpressionKind::Subtract => Type::None,
            ExpressionKind::Add => Type::None,
            ExpressionKind::Divide => Type::None,
            ExpressionKind::Multiply => Type::None,
            ExpressionKind::Power => Type::None,
            ExpressionKind::Call => Type::None,
            ExpressionKind::Grouping => Type::None,
            ExpressionKind::Condition => Type::None,
            ExpressionKind::Inequality => Type::None,
            ExpressionKind::LessThan => Type::None,
            ExpressionKind::GreaterThan => Type::None,
            ExpressionKind::LessThanOrEqualTo => Type::None,
            ExpressionKind::GreaterThanOrEqualTo => Type::None,
            ExpressionKind::Number(number) => self.analyze_number(number)?,
            ExpressionKind::Identifier(identifier) => self.analyze_identifier(identifier)?,
            ExpressionKind::Uri(uri) => self.analyze_uri(uri)?,
        };

        self.types.insert(reference, kind.clone());

        Ok(kind)
    }

    fn analyze_number(&mut self, number: &grammar::Number<'a>) -> AnalysisResult {
        let value = match Number::try_from(*number) {
            Ok(v) => v,
            Err(e) => {
                self.report_error(ErrorKind::InvalidNumber(e));
                Number::default()
            }
        };
        let index = self.numbers.insert(number.as_str().into(), value);

        if index > u8::MAX as usize {
            self.report_error(ErrorKind::TooManyNumbers(index));
        }

        Ok(Type::constant_number(index))
    }

    fn analyze_uri(&mut self, uri: &Uri<'a>) -> AnalysisResult {
        let index = self.texts.insert(uri.as_str().into(), Text::from(*uri));

        if index > u8::MAX as usize {
            self.report_error(ErrorKind::TooManyUris(index));
        }

        Ok(Type::constant_number(index))
    }

    fn analyze_identifier(&mut self, identifier: &Identifier<'a>) -> AnalysisResult {
        match self.resolve_local(identifier.as_str()) {
            Some(local) => Ok(local),
            None => match self.resolve_capture(identifier.as_str()) {
                Some(capture) => Ok(capture),
                None => {
                    let index = self
                        .functions
                        .top_mut()
                        .push_local(identifier.as_str().into());

                    if index >= u8::MAX as usize {
                        self.report_error(ErrorKind::TooManyLocals(index));
                    }

                    Ok(Type::local(index))
                }
            },
        }
    }

    fn resolve_capture(&mut self, name: &str) -> Option<Type> {
        match self.functions.top().resolve_capture(name) {
            Some(capture) => {
                return Some(Type::capture(capture.index()));
            }
            None => {
                let mut iterator = self.functions.iter_mut().rev().peekable();

                while let Some(enclosing) = iterator.next() {
                    if let Some(local) = enclosing.resolve_local_mut(name) {
                        let index = iterator.peek_mut()?.capture_local(local);

                        if index > u8::MAX as usize {
                            self.reporter
                                .report_analysis_error(ErrorKind::TooManyCaptures(index).into());
                        }

                        break;
                    }
                }

                while let Some(enclosing) = iterator.next() {
                    if let Some(function) = iterator.peek_mut() {
                        let capture = enclosing.resolve_capture(name)?;
                        let index = function.capture_transitive(capture);

                        if index > u8::MAX as usize {
                            self.reporter
                                .report_analysis_error(ErrorKind::TooManyCaptures(index).into());
                        }
                    } else {
                        enclosing.resolve_capture(name)?;
                    }
                }
            }
        }

        let capture = self.functions.top().resolve_capture(name)?;

        Some(Type::capture(capture.index()))
    }

    fn resolve_local(&mut self, name: &str) -> Option<Type> {
        let local = self.functions.top().resolve_local(name)?;

        if !local.initialized() {
            self.reporter
                .report_analysis_error(ErrorKind::UninitializedLocal(local.index()).into());
        }

        Some(Type::local(local.index()))
    }

    fn report_error<E: Into<AnalysisError>>(&mut self, error: E) {
        self.reporter.report_analysis_error(error.into());
    }
}

impl<'a> TryFrom<&'a str> for Analysis<'a> {
    type Error = Vec<CompilationError>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let program = Program::try_from(value)?;
        let analyzer = SemanticAnalyzer::new(vec![]);

        analyzer.analyze(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unused() {
        assert_eq!(
            Analysis::try_from("4ad5").unwrap_err(),
            vec![CompilationError::from(AnalysisError::from(
                ErrorKind::UnusedExpression
            ))]
        );
    }

    #[test]
    fn assignment() {
        assert!(Analysis::try_from("x = 42\nx")
            .unwrap()
            .is_assignment(&ExpressionReference(0)));
    }

    #[test]
    fn kind() {
        let analysis = Analysis::try_from("x = 42\nx").unwrap();

        assert_eq!(
            analysis.kind(&ExpressionReference(0)),
            &Type::constant_number(0)
        );
        assert!(analysis
            .kind(&ExpressionReference(0))
            .converts_to(&Type::number()));
    }
}
