//! Analyze the syntax tree to determine the offsets for all locals and captures within a scope.

mod error;

use crate::collections::{IndexedSet, NonEmptyStack};
use crate::grammar::ExpressionKind;
use crate::{ErrorReporter, SyntaxTree};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Function<'a> {
    name: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Local<'a> {
    name: &'a str,
}

impl<'a> Local<'a> {
    pub fn new(name: &'a str, _: usize) -> Self {
        Local { name }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Capture<'a> {
    name: &'a str,
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
}

pub struct ScopeAnalysis<'a> {
    scopes: Vec<Scope<'a>>,
    program: SyntaxTree<'a>,
}

impl<'a> ScopeAnalysis<'a> {
    pub fn new(program: SyntaxTree<'a>) -> Self {
        ScopeAnalysis {
            scopes: vec![],
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
        }
    }
}

impl<'a, R> ScopeAnalyzer<'a, R>
where
    R: ErrorReporter,
{
    pub fn analyze(mut self, mut analysis: ScopeAnalysis<'a>) -> Result<ScopeAnalysis<'a>, R> {
        // TODO: add scope index iterator.
        for node in analysis.program.iter() {
            let expression = node.data();

            match expression.kind() {
                ExpressionKind::Block if node.discovered() => {
                    let scope = match self.stack.pop() {
                        None => todo!("Report error."),
                        Some(s) => s,
                    };
                    let index = scope.index;

                    analysis.scopes[index] = scope;
                }
                ExpressionKind::Block => {
                    let index = analysis.scopes.len();
                    let scope = Scope::from(index);

                    analysis.scopes.push(scope.clone());

                    self.stack.push(scope);
                }
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
                _ => {}
            }
        }

        match self.stack.try_pop() {
            Err(scope) => analysis.scopes[0] = scope,
            Ok(..) => todo!("Report error."),
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(analysis)
        }
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
