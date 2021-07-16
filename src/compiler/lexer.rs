use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;
use std::io::Read;

pub fn tokenize<I: Read>(input: I) -> Result<Vec<Token>, CompilerError> {
    let contents: Node = serde_yaml::from_reader(input)?;

    Ok(vec![Token {
        index: 0,
        line: 0,
        column: 0,
        kind: TokenKind::YAML(contents),
    }])
}

/// Lexicographical tokens for Tortuga.
pub struct Token {
    index: usize,
    line: usize,
    column: usize,
    pub kind: TokenKind,
}

/// Type of tokens for Tortuga.
pub enum TokenKind {
    YAML(Node),
}
