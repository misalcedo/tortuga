use crate::analyzer::AnalyticalError;
use crate::error::CompilationError;
use crate::parser::SyntacticalError;
use crate::scanner::LexicalError;

pub trait ErrorReporter {
    fn report_lexical_error(&mut self, error: LexicalError);
    fn report_syntax_error(&mut self, error: SyntacticalError);
    fn report_analysis_error(&mut self, error: AnalyticalError);
}

impl ErrorReporter for Vec<CompilationError> {
    fn report_lexical_error(&mut self, error: LexicalError) {
        self.push(error.into())
    }

    fn report_syntax_error(&mut self, error: SyntacticalError) {
        self.push(error.into())
    }

    fn report_analysis_error(&mut self, error: AnalyticalError) {
        self.push(error.into())
    }
}
