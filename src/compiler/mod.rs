use crate::compiler::errors::CompilerError;
use std::io::{Read, Write};

mod emitter;
mod errors;
mod lexer;
mod parser;
mod transformer;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile<I: Read, O: Write>(&self, input: I, output: O) -> Result<usize, CompilerError> {
        Ok(0)
    }
}
