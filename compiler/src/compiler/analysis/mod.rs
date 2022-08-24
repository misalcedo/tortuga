use crate::{CompilationError, ErrorReporter, Program};
use std::collections::{HashMap, HashSet};

mod capture;
mod error;
mod function;
mod local;
mod result;
mod types;

use crate::collections::NonEmptyStack;
use crate::grammar::{
    ExpressionKind, ExpressionReference, Identifier, Node, ReferenceIterator, Uri,
};
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
        Ok(Type::Number(None))
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
}
