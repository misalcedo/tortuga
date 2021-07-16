mod emitter;
mod errors;
mod lexer;
mod parser;
mod transformer;

pub use errors::CompilerError;
use std::io::{Read, Write};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile<I: Read, O: Write>(
        &self,
        input: &I,
        output: &mut O,
    ) -> Result<usize, CompilerError> {
        let tokens = lexer::tokenize(input)?;
        let ast = parser::parse(&tokens)?;
        let target = transformer::transform(&ast)?;

        emitter::emit_binary(target, output)
    }
}
