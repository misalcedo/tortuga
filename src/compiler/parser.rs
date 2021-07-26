use crate::compiler::lexer::{Token, TokenKind};
use crate::compiler::CompilerError;
use crate::syntax::tortuga::Process;

#[tracing::instrument]
pub async fn parse(tokens: &[Token]) -> Result<Process, CompilerError> {
    match tokens.first() {
        Some(Token {
            kind: TokenKind::Yaml(value),
            ..
        }) => {
            let node: Process = serde_yaml::from_value(value.clone())?;

            tracing::trace!("Parsed a syntax tree from YAML.");

            Ok(node)
        }
        None => Ok(Process::default()),
    }
}
