use crate::compiler::CompilerError;
use futures::{AsyncRead, AsyncReadExt};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fmt::Debug;

#[tracing::instrument]
pub async fn tokenize<I: AsyncRead + Debug + Unpin>(
    input: &mut I,
) -> Result<Vec<Token>, CompilerError> {
    let mut buffer = Vec::new();

    input.read_to_end(&mut buffer).await?;

    tracing::debug!("Read {} bytes.", buffer.len());

    match serde_yaml::from_reader::<&[u8], Value>(&buffer[..]) {
        Ok(value) => Ok(vec![Token {
            kind: TokenKind::Yaml(value),
        }]),
        Err(_) => Ok(vec![]),
    }
}

/// Lexicographical tokens for Tortuga.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
}

/// Type of tokens for Tortuga.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Yaml(Value),
}
