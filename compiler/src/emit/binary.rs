use crate::executable::Executable;
use crate::grammar::WithoutScopeDepth;
use crate::Program;
use std::io::Write;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct BinaryEmitter {}

impl BinaryEmitter {
    pub fn emit<W: Write>(&self, executable: &Executable, mut output: W) -> std::io::Result<()> {
        Ok(())
    }
}
