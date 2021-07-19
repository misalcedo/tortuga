use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;
use futures::AsyncRead;

pub async fn tokenize<I: AsyncRead>(input: I) -> Result<Vec<Token>, CompilerError> {
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
