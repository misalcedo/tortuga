//! Perform semantic analysis on a program.

use crate::collections::{IndexedSet, NonEmptyStack};
use crate::grammar::Identifier;
use crate::{ErrorReporter, Program};

pub struct UnusedLocalAnalysis<'a> {
    scopes: NonEmptyStack<IndexedSet<Identifier<'a>>>,
}

impl<'a> UnusedLocalAnalysis<'a> {
    fn analyze<E>(mut self, program: &Program<'a>, reporter: E) -> Result<Self, E>
    where
        E: ErrorReporter,
    {
        Err(reporter)
    }
}
