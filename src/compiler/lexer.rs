use crate::compiler::CompilerError;
use futures::{AsyncRead, AsyncReadExt};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[tracing::instrument]
pub async fn tokenize<I: AsyncRead + Debug + Unpin>(
    input: &mut I,
) -> Result<Vec<Token>, CompilerError> {
    let mut buffer = Vec::new();

    input.read_to_end(&mut buffer).await?;

    tracing::debug!("Read {} bytes.", buffer.len());

    let value = serde_yaml::from_reader::<&[u8], serde_yaml::Value>(&buffer[..]);

    match value {
        Ok(_) => Ok(vec![Token {
            kind: TokenKind::Yaml(buffer),
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
    Yaml(Vec<u8>),
}
