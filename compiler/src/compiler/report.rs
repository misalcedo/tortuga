use crate::compiler::error::CompilationError;
use crate::compiler::parse::SyntaxError;
use crate::compiler::scan::LexicalError;
use crate::compiler::scope::ScopeError;

pub trait ErrorReporter {
    fn report_lexical_error(&mut self, error: LexicalError);
    fn report_syntax_error(&mut self, error: SyntaxError);
    fn report_scope_error(&mut self, error: ScopeError);
    fn had_error(&self) -> bool;
}

impl ErrorReporter for Vec<CompilationError> {
    fn report_lexical_error(&mut self, error: LexicalError) {
        self.push(error.into())
    }

    fn report_syntax_error(&mut self, error: SyntaxError) {
        self.push(error.into())
    }

    fn report_scope_error(&mut self, error: ScopeError) {
        self.push(error.into())
    }

    fn had_error(&self) -> bool {
        !self.is_empty()
    }
}
