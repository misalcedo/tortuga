mod module;
mod types;
mod values;

use crate::compiler::errors::CompilerError;
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

#[cfg(test)]
mod tests {
    use super::BinaryWebAssemblyEmitter;
    use crate::compiler::emitter::Emitter;
    use crate::web_assembly::*;
    use std::io::Cursor;

    #[test]
    fn empty_module() {
        let mut buffer = Cursor::new(Vec::new());
        let emitter = BinaryWebAssemblyEmitter::new();
        let module = Module::new();

        emitter.emit(&module, &mut buffer);

        assert_eq!(buffer.get_ref(), b"\x00\x61\x73\x6D\x01\x00\x00\x00")
    }
}
