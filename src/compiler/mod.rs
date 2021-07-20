mod emitter;
mod errors;
mod lexer;
mod parser;
mod transformer;

pub use errors::CompilerError;
use futures::{AsyncRead, AsyncWrite};

struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Compiler {}
    }

    pub async fn compile<I: AsyncRead + Unpin, O: AsyncWrite + Unpin>(
        &self,
        input: &mut I,
        output: &mut O,
    ) -> Result<usize, CompilerError> {
        let tokens = lexer::tokenize(input).await?;
        let ast = parser::parse(&tokens).await?;
        let target = transformer::transform(&ast).await?;

        emitter::emit_binary(&target, output).await
    }
}

// TODO: learn about pin and unpin, support async read in lexer, fix emit_vector implementation.
pub async fn compile<I: AsyncRead + Unpin, O: AsyncWrite + Unpin>(
    input: &mut I,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let compiler = Compiler::new();
    let tokens = lexer::tokenize(input).await?;
    let ast = parser::parse(&tokens).await?;
    let target = transformer::transform(&ast).await?;

    emitter::emit_binary(&target, output).await
}
