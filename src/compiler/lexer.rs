use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;
use futures::{AsyncRead, AsyncReadExt};

pub async fn tokenize<I: AsyncRead + Unpin>(input: &mut I) -> Result<Vec<Token>, CompilerError> {
    let mut buffer = Vec::new();

    input.read_to_end(&mut buffer);

    let contents: Node = serde_yaml::from_reader(&buffer[..])?;

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
