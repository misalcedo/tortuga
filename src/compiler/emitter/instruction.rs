use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::Expression;
use std::io::Write;

impl Emit for Expression {
    fn emit<O: Write>(&self, output: O) -> Result<usize, CompilerError> {
        Ok(0)
    }
}
