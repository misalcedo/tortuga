use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;
use std::io::Read;

pub async fn tokenize<I: Read>(input: I) -> Result<Vec<Token>, CompilerError> {
    let contents: Node = serde_yaml::from_reader(input)?;

    Ok(vec![Token {
        kind: TokenKind::YAML(contents),
    }])
}

/// Lexicographical tokens for Tortuga.
pub struct Token {
    pub kind: TokenKind,
}

/// Type of tokens for Tortuga.
pub enum TokenKind {
    YAML(Node),
}
