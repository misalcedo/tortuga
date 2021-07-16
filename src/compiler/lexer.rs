use crate::compiler::CompilerError;
use std::io::Read;

pub fn tokenize<I: Read>(_input: &I) -> Result<Vec<Token>, CompilerError> {
    Ok(Vec::new())
}

/// Lexicographical tokens for Tortuga.
pub struct Token {
    line: usize,
    column: usize,
    kind: TokenKind,
}

/// Type of tokens for Tortuga.
pub enum TokenKind {}
