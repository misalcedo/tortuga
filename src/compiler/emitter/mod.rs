mod instruction;
mod module;
mod types;
mod values;

use crate::compiler::errors::CompilerError;
use std::io::Write;

pub use types::*;
pub use values::*;

/// Emits a binary representation of a WebAssembly Abstract Syntax Tree (AST) to a `Write` output.
pub trait Emit {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError>;
}

#[cfg(test)]
mod tests {
    use crate::compiler::emitter::Emit;
    use crate::web_assembly::*;
    use std::io::Cursor;

    #[test]
    fn empty_module() {
        let mut buffer = Cursor::new(Vec::new());
        let module = Module::new();

        module.emit(&mut buffer).unwrap();

        assert_eq!(buffer.get_ref(), b"\x00\x61\x73\x6D\x01\x00\x00\x00")
    }
}
