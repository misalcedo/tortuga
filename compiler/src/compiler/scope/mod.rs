use crate::collections::{Forest, IndexedSet, NonEmptyStack};
use crate::grammar::{Expression, ExpressionKind};
use crate::{ErrorReporter, SyntaxTree};

#[derive(Clone, Debug)]
pub struct Local<'a> {
    name: &'a str,
    scope: usize,
    captured: bool,
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
                ExpressionKind::Identifier => {
                    let scope = self.stack.top_mut();
                    let name = expression.as_str();

                    scope.locals.contains(name);
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
