use crate::compiler::lexer::Token;
use crate::compiler::CompilerError;
use crate::syntax::tortuga::Node;

pub fn parse(_tokens: &[Token]) -> Result<Node, CompilerError> {
    Ok(Node::new())
}
