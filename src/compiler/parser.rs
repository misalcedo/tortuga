use crate::compiler::lexer::{Token, TokenKind};
use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;

pub fn parse(tokens: &[Token]) -> Result<Node, CompilerError> {
    match tokens.first() {
        Some(Token {
            kind: TokenKind::YAML(node),
            ..
        }) => Ok(node.clone()),
        None => Ok(Node::new()),
    }
}
