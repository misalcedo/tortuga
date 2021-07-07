use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{Expression, Instruction};
use byteorder::WriteBytesExt;
use std::io::Write;
use std::mem::size_of;

impl Emit for Expression {
    fn emit<O: Write>(&self, mut output: O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        for instruction in self.instructions() {
            bytes += instruction.emit(&mut output)?;
        }

        output.write_u8(0x0B)?;

        Ok(bytes)
    }
}

impl Emit for Instruction {
    fn emit<O: Write>(&self, output: O) -> Result<usize, CompilerError> {
        Ok(0)
    }
}
