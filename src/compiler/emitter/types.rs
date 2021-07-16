use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{
    FunctionType, GlobalType, Limit, MemoryType, NumberType, ReferenceType, ResultType, TableType,
    ValueType,
};
use std::io::Write;

impl Emit for NumberType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let value: u8 = match self {
            NumberType::I32 => 0x7F,
            NumberType::I64 => 0x7E,
            NumberType::F32 => 0x7D,
            NumberType::F64 => 0x7C,
        };

        value.emit(output)
    }
}

impl Emit for ReferenceType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let value: u8 = match self {
            ReferenceType::Function => 0x70,
            ReferenceType::External => 0x6F,
        };

        value.emit(output)
    }
}

impl Emit for ValueType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            ValueType::Number(number_type) => number_type.emit(output),
            ValueType::Reference(reference_type) => reference_type.emit(output),
        }
    }
}

impl Emit for ResultType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.kinds().emit(output)
    }
}

impl Emit for FunctionType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += 0x60u8.emit(output)?;
        bytes += self.parameters().emit(output)?;
        bytes += self.results().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for Limit {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self.max() {
            Some(max) => {
                bytes += 0x01u8.emit(output)?;
                bytes += self.min().emit(output)?;
                bytes += max.emit(output)?;
            }
            None => {
                bytes += 0x00u8.emit(output)?;
                bytes += self.min().emit(output)?;
            }
        };

        Ok(bytes)
    }
}

impl Emit for MemoryType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.limits().emit(output)
    }
}

impl Emit for TableType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.kind().emit(output)?;
        bytes += self.limits().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for GlobalType {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.kind().emit(output)?;

        let mutability: u8 = match self.is_mutable() {
            false => 0x00,
            true => 0x01,
        };

        bytes += mutability.emit(output)?;

        Ok(bytes)
    }
}
