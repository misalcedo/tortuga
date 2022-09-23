//! Analyze the syntax tree to determine the offsets for all locals and captures within a scope.

mod visitor;

use crate::collections::tree::Node;
use crate::collections::{Forest, IndexedSet, NonEmptyStack};
use crate::grammar::{Expression, ExpressionKind};
use crate::{ErrorReporter, SyntaxTree};

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Debug)]
pub struct Capture<'a> {
    name: &'a str,
    transitive: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Scope<'a> {
    depth: usize,
    index: usize,
    locals: IndexedSet<&'a str, Local<'a>>,
    captures: IndexedSet<&'a str, Capture<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(depth: usize, index: usize) -> Self {
        Scope {
            depth,
            index,
            locals: Default::default(),
            captures: Default::default(),
        }
    }
}

pub struct ScopeAnalyzer<'a, Reporter> {
    program: SyntaxTree<'a>,
    reporter: Reporter,
    stack: NonEmptyStack<Scope<'a>>,
    scopes: Vec<Vec<Scope<'a>>>,
}

pub struct ScopeAnalysis<'a> {
    scopes: Vec<Vec<Scope<'a>>>,
    program: SyntaxTree<'a>,
}

impl<'a, R> ScopeAnalyzer<'a, R>
where
    R: ErrorReporter,
{
    pub fn new(program: SyntaxTree<'a>, reporter: R) -> Self {
        ScopeAnalyzer {
            program,
            reporter,
            stack: Default::default(),
            scopes: vec![vec![]],
        }
    }

    pub fn analyze(mut self) -> Result<ScopeAnalysis<'a>, R> {
        // TODO: add scope index iterator.
        for node in self.program.iter() {
            let expression = node.data();

            match expression.kind() {
                ExpressionKind::Block if node.discovered() => {
                    let scope = match self.stack.pop() {
                        None => todo!("Report error."),
                        Some(s) => s,
                    };
                    let index = scope.index;
                    let depth = scope.depth;

                    self.scopes[depth][index] = scope;
                }
                ExpressionKind::Block => {
                    let depth = self.stack.len();
                    let index = self.scopes.get(depth).map(Vec::len).unwrap_or_default();
                    let scope = Scope::new(depth, index);

                    if depth >= self.scopes.len() {
                        self.scopes.push(vec![scope.clone()])
                    } else {
                        self.scopes[depth].push(scope.clone());
                    }

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

                            scope.locals.insert(name, Local::new(name, scope.depth));
                        }
                        ExpressionKind::Identifier => {
                            let name = target.data().as_str();

                            scope.locals.insert(name, Local::new(name, scope.depth));
                        }
                        ExpressionKind::Grouping => {
                            let kinds: Vec<ExpressionKind> =
                                target.children().map(|n| *n.data().kind()).collect();

                            if kinds.iter().all(|c| c == &ExpressionKind::Identifier) {
                                for child in target.children() {
                                    let name = child.data().as_str();

                                    scope.locals.insert(name, Local::new(name, scope.depth));
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
            Err(scope) => self.scopes[0].push(scope),
            Ok(..) => todo!("Report error."),
        }

        if self.reporter.had_error() {
            Err(self.reporter)
        } else {
            Ok(ScopeAnalysis {
                scopes: self.scopes,
                program: self.program,
            })
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
        let analyzer = ScopeAnalyzer::new(program, vec![]);
        let analysis: ScopeAnalysis<'_> = analyzer.analyze().unwrap();

        for scope in analysis.scopes {
            println!("{:?}", scope);
        }

        assert!(false);
    }
}
