//! Analyses a program for the following:
//! - Unused variables.
//! - Define a variable before using.
//! - Type checking.

mod capture;
mod constants;
mod error;
mod local;
mod scope;

use crate::grammar::{
    Expression, ExpressionReference, Identifier, Internal, InternalKind, Number, PreOrderIterator,
    Program, Terminal, Uri,
};
use crate::{CompilationError, ErrorReporter};
use constants::ConstantAnalysis;
pub use error::AnalyticalError;
use scope::ScopeContext;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Analyzer<'a, Reporter> {
    program: Program<'a>,
    reporter: Reporter,
    had_error: bool,
    scopes: Vec<ScopeContext<'a>>,
    stack: Vec<ValueKind<'a>>,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum ValueKind<'a> {
    Identifier(Identifier<'a>),
    Number,
    Signature(Identifier<'a>),
    Function(Identifier<'a>),
    Uri,
    Grouping(Vec<Self>),
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Local<'a> {
    kind: ValueKind<'a>,
    depth: usize,
    index: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Analysis<'a> {
    program: Program<'a>,
    constants: ConstantAnalysis<'a>,
}

type AnalyticalResult<Output> = Result<Output, AnalyticalError>;

impl<'a, R> Analyzer<'a, R>
where
    R: ErrorReporter,
{
    fn new(program: Program<'a>, reporter: R) -> Self {
        Analyzer {
            program,
            reporter,
            had_error: false,
            scopes: Default::default(),
            stack: Default::default(),
        }
    }

    pub fn analyze(mut self) -> Result<Analysis<'a>, R> {
        let constants = ConstantAnalysis::from(&self.program);
        let mut iterator = self.program.iter_post_order();

        // self.scopes.push(ScopeContext::default());
        //
        // while let Some((depth, expression)) = iterator.next() {
        //     match expression {
        //         Expression::Internal(internal) => {
        //             match internal.kind() {
        //                 InternalKind::Equality => {
        //                     let value = match self.stack.pop() {
        //                         Some(v) => v,
        //                         None => {
        //                             self.reporter.report_analysis_error(AnalyticalError::new("Encountered an equality expression without a right-hand side."));
        //                             break;
        //                         }
        //                     };
        //
        //                     let assignee = match self.stack.pop() {
        //                         Some(k) => k,
        //                         None => {
        //                             self.reporter.report_analysis_error(AnalyticalError::new("Encountered an equality expression without a left-hand side."));
        //                             break;
        //                         }
        //                     };
        //
        //                     match assignee {
        //                         ValueKind::Identifier(i) => match self.scopes.last() {
        //                             Some(mut scope) => scope.add_local(i),
        //                             None => {
        //                                 self.had_error = true;
        //                                 self.reporter.report_analysis_error(AnalyticalError::new(
        //                                     "Found identifier outside of scope.",
        //                                 ))
        //                             }
        //                         },
        //                         ValueKind::Number => self.stack.push(ValueKind::Number),
        //                         ValueKind::Signature(_) => {}
        //                         ValueKind::Function(_) => self.stack.push(ValueKind::Number),
        //                         ValueKind::Uri => self.stack.push(ValueKind::Number),
        //                         ValueKind::Grouping(_) => self.stack.push(ValueKind::Number),
        //                     }
        //                 }
        //                 _ => (),
        //             }
        //         }
        //         Expression::Terminal(terminal) => match terminal {
        //             Terminal::Number(number) => {
        //                 self.stack.push(ValueKind::Number);
        //             }
        //             Terminal::Uri(uri) => {
        //                 self.stack.push(ValueKind::Uri);
        //             }
        //             Terminal::Identifier(identifier) => {
        //                 let kind = self
        //                     .locals
        //                     .get(identifier)
        //                     .map(|l| l.kind.clone())
        //                     .unwrap_or_else(|| ValueKind::Identifier(*identifier));
        //
        //                 self.stack.push(kind);
        //             }
        //         },
        //     };
        // }

        if self.had_error {
            Err(self.reporter)
        } else {
            Ok(Analysis {
                program: self.program,
                constants,
            })
        }
    }
}

impl<'a> TryFrom<&'a str> for Analysis<'a> {
    type Error = Vec<CompilationError>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let program = Program::try_from(input)?;

        Analyzer::new(program, Vec::default()).analyze()
    }
}

impl<'a> Display for ValueKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::Identifier(name) => write!(f, "{}", name),
            ValueKind::Uri => write!(f, "{:?}", self),
            ValueKind::Number => write!(f, "{:?}", self),
            ValueKind::Signature(name) => write!(f, "{}()", name),
            ValueKind::Function(name) => write!(f, "<{}>", name),
            ValueKind::Grouping(parts) => {
                write!(
                    f,
                    "({})",
                    parts
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

impl<'a> Display for Local<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Kind: {}, Depth: {}, Index: {}",
            self.kind, self.depth, self.index
        )
    }
}

impl<'a> Display for Analysis<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# Program Analysis")?;
        writeln!(f, "{}", self.constants)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undefined_variable() {
        assert!(!Analysis::try_from("x + 42").unwrap_err().is_empty());
    }

    #[test]
    fn add_wrong_types() {
        assert!(!Analysis::try_from("x = \"Hello\"\nx + 42")
            .unwrap_err()
            .is_empty());
    }

    #[test]
    fn undefined() {
        assert!(
            !Analysis::try_from(include_str!("../../../examples/undefined.ta"))
                .unwrap_err()
                .is_empty()
        );
    }

    #[test]
    fn factorial() {
        let analysis = Analysis::try_from(include_str!("../../../examples/factorial.ta")).unwrap();

        println!("{}", analysis);
        assert!(false);
    }
}
