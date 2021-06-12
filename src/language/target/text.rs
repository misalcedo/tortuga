use std::io::Write;
use crate::language::target::emitter::{Emitter, EmitterError, ErrorKind};
use crate::language::target::model::{self, Module, Type, FunctionType, Param, ValueType};

struct TextEmitter {
}

impl TextEmitter {
    /// Create a new WebAssembly text emitter.
    pub fn new() -> TextEmitter {
        TextEmitter {
        }
    }
}

impl Emitter for TextEmitter {
    fn write(&self, module: &Module, output: &mut impl Write) -> Result<(), EmitterError> {
        match module.name {
            Some(ref name) => {
                write!(output, "(module ${})", name.value).map_err(|_| EmitterError::new(ErrorKind::WriteFailure))?;
            },
            None => {
                write!(output, "(module)").map_err(|_| EmitterError::new(ErrorKind::WriteFailure))?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn empty_module() {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut emitter = TextEmitter::new();

        let module = Module::new(Option::None);
        let result = emitter.write(&module, &mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.len(), b"(module)".len())
    }

    #[test]
    fn empty_module_with_name() {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut emitter = TextEmitter::new();

        let module = Module::new(Option::Some("test"));
        let result = emitter.write(&module, &mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.len(), b"(module $test)".len())
    }

    #[test]
    fn module_with_type() {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut emitter = TextEmitter::new();

        let mut module = Module::new(Option::Some("test"));
        let kind = Type::new(
            Option::Some("foo"),
     FunctionType::new(vec![
                    Param::new(Option::Some("a"), ValueType::I32),
                    Param::new(Option::Some("a"), ValueType::F64)],
                    vec![model::Result::new(ValueType::I64)]
            )
        );
        let result = emitter.write(&module, &mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer.len(), b"(module $test (type $foo (func (param $a i32) (param f64) (result i64))))".len())
    }
}