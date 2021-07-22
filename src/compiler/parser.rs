use crate::compiler::lexer::{Token, TokenKind};
use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;

#[tracing::instrument]
pub async fn parse(tokens: &[Token]) -> Result<Node, CompilerError> {
    match tokens.first() {
        Some(Token {
            kind: TokenKind::Yaml(buffer),
            ..
        }) => {
            let node: Node = serde_yaml::from_reader(&buffer[..])?;

            tracing::trace!("Parsed a syntax tree from YAML.");

            Ok(node)
        }
        None => Ok(Node::new()),
    }
}
