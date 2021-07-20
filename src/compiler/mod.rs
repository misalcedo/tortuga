mod emitter;
mod errors;
mod lexer;
mod parser;
mod transformer;

pub use errors::CompilerError;
use std::io::{Read, Write};

pub async fn compile<I: Read, O: Write>(input: I, output: &mut O) -> Result<usize, CompilerError> {
    let tokens = lexer::tokenize(input).await?;
    let ast = parser::parse(&tokens).await?;
    let target = transformer::transform(&ast).await?;

    emitter::emit_binary(&target, output).await
}
