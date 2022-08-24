use crate::collections::NonEmptyStack;
use crate::compiler::analysis::{Function, SemanticAnalyzer, Type};
use crate::grammar::ExpressionReference;
use crate::Program;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Analysis<'a> {
    input: Program<'a>,
    assignments: HashSet<ExpressionReference>,
    functions: NonEmptyStack<Function<'a>>,
    types: HashMap<ExpressionReference, Type>,
}

impl<'a> Analysis<'a> {
    pub fn new<R>(program: Program<'a>, analyzer: SemanticAnalyzer<'a, R>) -> Self {
        Analysis {
            input: program,
            assignments: analyzer.assignments,
            functions: analyzer.functions,
            types: analyzer.types,
        }
    }

    pub fn is_assignment(&self, expression: &ExpressionReference) -> bool {
        self.assignments.contains(expression)
    }

    pub fn kind(&self, expression: &ExpressionReference) -> &Type {
        self.types.get(expression).unwrap_or(&Type::None)
    }
}
