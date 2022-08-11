use crate::error::CompilationError;
use crate::parse::SyntacticalError;
use crate::scan::LexicalError;
use crate::translate::TranslationError;

pub trait ErrorReporter {
    fn report_lexical_error(&mut self, error: LexicalError);
    fn report_syntax_error(&mut self, error: SyntacticalError);
    fn report_translation_error(&mut self, error: TranslationError);
    fn had_error(&self) -> bool;
}

impl ErrorReporter for Vec<CompilationError> {
    fn report_lexical_error(&mut self, error: LexicalError) {
        self.push(error.into())
    }

    fn report_syntax_error(&mut self, error: SyntacticalError) {
        self.push(error.into())
    }

    fn report_translation_error(&mut self, error: TranslationError) {
        self.push(error.into())
    }

    fn had_error(&self) -> bool {
        !self.is_empty()
    }
}
