mod types;
mod values;

use crate::compiler::errors::CompilerError;
use crate::web_assembly::Module;
use std::io::Write;

pub use types::*;
pub use values::*;

/// Emits a representation of an Abstract Syntax Tree (AST) to a `Write` output.
pub trait Emitter<T: ?Sized> {
    fn emit<O: Write>(&self, node: &T, output: O) -> Result<usize, CompilerError>;
}

pub struct BinaryWebAssemblyEmitter {}

impl BinaryWebAssemblyEmitter {
    pub fn new() -> BinaryWebAssemblyEmitter {
        BinaryWebAssemblyEmitter {}
    }
}

/// See https://webassembly.github.io/spec/core/binary/modules.html
impl Emitter<Module> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, module: &Module, output: O) -> Result<usize, CompilerError> {
        Ok(0)
    }
}
