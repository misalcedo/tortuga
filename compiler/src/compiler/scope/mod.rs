//! Analyze the syntax tree to determine the offsets for all locals and captures within a scope.

mod error;

use crate::collections::tree::Node;
use crate::collections::{IndexedSet, NonEmptyStack};
use crate::compiler::Excerpt;
use crate::grammar::{Expression, ExpressionKind};
use crate::{ErrorReporter, SyntaxTree};
pub use error::{ScopeError, ScopeErrorKind};
use std::collections::HashMap;
use std::iter::Peekable;

const STATEMENT_KINDS: &[ExpressionKind] = &[ExpressionKind::Equality];
type Scopes<'a> = HashMap<usize, Scope<'a>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Function<'a> {
    name: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Local<'a> {
    name: &'a str,
    scope: usize,
    captured: bool,
}

impl<'a> Local<'a> {
    pub fn new(name: &'a str, scope: usize) -> Self {
        Local {
            name,
            scope,
            captured: false,
        }
    }

    pub fn capture(&mut self) -> Capture<'a> {
        self.captured = true;

        Capture {
            name: self.name,
            scope: self.scope,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Capture<'a> {
    name: &'a str,
    scope: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scope<'a> {
    index: usize,
    functions: IndexedSet<&'a str, Function<'a>>,
    locals: IndexedSet<&'a str, Local<'a>>,
    captures: IndexedSet<&'a str, Capture<'a>>,
}

impl<'a> From<usize> for Scope<'a> {
    fn from(index: usize) -> Self {
        Scope {
            index,
            functions: Default::default(),
            locals: Default::default(),
            captures: Default::default(),
        }
    }
}

pub struct ScopeAnalyzer<'a, Reporter> {
    reporter: Reporter,
    stack: NonEmptyStack<Scope<'a>>,
    index: usize,
}

pub struct ScopeAnalysis<'a> {
    scopes: HashMap<usize, Scope<'a>>,
    program: SyntaxTree<'a>,
}

impl<'a> ScopeAnalysis<'a> {
    pub fn new(program: SyntaxTree<'a>) -> Self {
        ScopeAnalysis {
            scopes: HashMap::new(),
            program,
        }
    }
}

impl<'a, R> From<R> for ScopeAnalyzer<'a, R>
where
    R: ErrorReporter,
{
    fn from(reporter: R) -> Self {
        ScopeAnalyzer {
            reporter,
            stack: Default::default(),
            index: 0,
        }
    }
}

impl<'a, R> ScopeAnalyzer<'a, R>
where
    R: ErrorReporter,
{
    pub fn analyze(mut self, mut analysis: ScopeAnalysis<'a>) -> Result<ScopeAnalysis<'a>, R> {
        let result = self.analyze_block(&mut analysis.scopes, analysis.program.roots());

        if let Err(error) = result {
            self.reporter.report_scope_error(error);
        }

        match self.stack.try_pop() {
            Err(scope) => {
                analysis.scopes.insert(0, scope);
            }
            Ok(..) => self.reporter.report_scope_error(ScopeError::new(
                ScopeErrorKind::UnterminatedScope,
                Excerpt::default(),
            )),
        };

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(analysis)
        }
    }

    fn analyze_block<'b, I>(
        &mut self,
        scopes: &mut Scopes<'a>,
        iterator: I,
    ) -> Result<(), ScopeError>
    where
        'a: 'b,
        I: Iterator<Item = Node<'b, Expression<'a>>>,
    {
        self.enter_scope();

        let mut iterator = iterator.peekable();

        while let Some(node) = iterator.next() {
            if iterator.peek().is_some() {
                self.analyze_statement(scopes, node)?
            } else {
                self.analyze_expression(scopes, node)?
            }
        }

        self.exit_scope(scopes)
    }

    fn analyze_statement(
        &mut self,
        scopes: &mut Scopes<'a>,
        node: Node<Expression<'a>>,
    ) -> Result<(), ScopeError> {
        if !STATEMENT_KINDS.contains(node.data().kind()) {
            self.reporter.report_scope_error(ScopeError::new(
                ScopeErrorKind::UnusedExpressionValue,
                Excerpt::default(),
            ));
        }

        self.analyze_expression(scopes, node)
    }

    fn analyze_expression(
        &mut self,
        scopes: &mut Scopes<'a>,
        node: Node<Expression<'a>>,
    ) -> Result<(), ScopeError> {
        let expression = node.data();

        match expression.kind() {
            ExpressionKind::Equality if !node.discovered() => {
                let scope = self.stack.top_mut();
                let mut children = node.children();
                let target = match children.next() {
                    None => todo!("Report error."),
                    Some(t) => t,
                };
                let value = match children.next() {
                    None => todo!("Report error."),
                    Some(t) => t,
                };
                let condition = children.next();

                if condition.is_some() && target.data().kind() != &ExpressionKind::Call {
                    todo!("Report error.");
                }

                match target.data().kind() {
                    ExpressionKind::Call => {
                        let child = match target.children().next() {
                            None => todo!("Report error."),
                            Some(t) if t.data().kind() != &ExpressionKind::Identifier => {
                                todo!("Report error.")
                            }
                            Some(t) => t,
                        };

                        let name = target.data().as_str();

                        scope.locals.insert(name, Local::new(name, scope.index));
                    }
                    ExpressionKind::Identifier => {
                        let name = target.data().as_str();

                        scope.locals.insert(name, Local::new(name, scope.index));
                    }
                    ExpressionKind::Grouping => {
                        let kinds: Vec<ExpressionKind> =
                            target.children().map(|n| *n.data().kind()).collect();

                        if kinds.iter().all(|c| c == &ExpressionKind::Identifier) {
                            for child in target.children() {
                                let name = child.data().as_str();

                                scope.locals.insert(name, Local::new(name, scope.index));
                            }
                        } else {
                            todo!("Report error.");
                        }
                    }
                    _ => todo!("Report error."),
                }
            }
            ExpressionKind::Identifier if node.discovered() => {
                let scope = self.stack.top_mut();
                let name = expression.as_str();

                if !scope.locals.contains(name) {
                    todo!("Report error.")
                }
            }
            kind => self.reporter.report_scope_error(ScopeError::new(
                ScopeErrorKind::UnsupportedExpression(*kind),
                Excerpt::default(),
            )),
        }

        Ok(())
    }

    fn exit_scope(&mut self, scopes: &mut Scopes<'a>) -> Result<(), ScopeError> {
        let scope = self
            .stack
            .pop()
            .ok_or_else(|| ScopeError::new(ScopeErrorKind::ExitRootScope, Excerpt::default()))?;

        scopes.insert(scope.index, scope);

        Ok(())
    }

    fn enter_scope(&mut self) {
        let scope = Scope::from(self.index);

        self.index += 1;
        self.stack.push(scope);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial() {
        let program =
            SyntaxTree::try_from(include_str!("../../../../examples/factorial.ta")).unwrap();
        let analyzer = ScopeAnalyzer::from(vec![]);
        let analysis: ScopeAnalysis<'_> = analyzer.analyze(ScopeAnalysis::new(program)).unwrap();

        for scope in analysis.scopes {
            println!("{:?}", scope);
        }

        assert!(false);
    }
}
