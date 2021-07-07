use crate::compiler::emitter::{BinaryWebAssemblyEmitter, Emitter};
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{
    FunctionType, GlobalType, Limit, MemoryType, Mutability, NumberType, ReferenceType, TableType,
    ValueType,
};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use std::mem::size_of;

impl Emitter<NumberType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        number_type: &NumberType,
        mut output: O,
    ) -> Result<usize, CompilerError> {
        let value: u8 = match number_type {
            NumberType::I32 => 0x7F,
            NumberType::I64 => 0x7E,
            NumberType::F32 => 0x7D,
            NumberType::F64 => 0x7C,
        };

        output.write_u8(value)?;

        Ok(size_of::<u8>())
    }
}

impl Emitter<ReferenceType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        reference_type: &ReferenceType,
        mut output: O,
    ) -> Result<usize, CompilerError> {
        let value: u8 = match reference_type {
            ReferenceType::Function => 0x70,
            ReferenceType::External => 0x6F,
        };

        output.write_u8(value)?;

        Ok(size_of::<u8>())
    }
}

impl Emitter<ValueType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, value_type: &ValueType, output: O) -> Result<usize, CompilerError> {
        match value_type {
            ValueType::Number(number_type) => self.emit(number_type, output),
            ValueType::Reference(reference_type) => self.emit(reference_type, output),
        }
    }
}

impl Emitter<[&ValueType]> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        value_types: &[&ValueType],
        mut output: O,
    ) -> Result<usize, CompilerError> {
        output.write_u32::<LittleEndian>(value_types.len() as u32)?;

        let mut bytes = size_of::<u32>();

        for value_type in value_types {
            bytes += self.emit(*value_type, &mut output)?;
        }

        Ok(bytes)
    }
}

impl Emitter<FunctionType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        function_type: &FunctionType,
        mut output: O,
    ) -> Result<usize, CompilerError> {
        output.write_u8(0x60)?;

        let mut bytes = size_of::<u8>();

        bytes += self.emit(&function_type.parameters()[..], &mut output)?;
        bytes += self.emit(&function_type.results()[..], &mut output)?;

        Ok(bytes)
    }
}

impl Emitter<Limit> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, limit: &Limit, mut output: O) -> Result<usize, CompilerError> {
        let mut bytes = size_of::<u8>();

        match limit {
            Limit::Min(min) => {
                output.write_u8(0x00)?;

                bytes += self.emit(min, &mut output)?;
            }
            Limit::MinMax { min, max } => {
                output.write_u8(0x01)?;

                bytes += self.emit(min, &mut output)?;
                bytes += self.emit(max, &mut output)?;
            }
        };

        Ok(bytes)
    }
}

impl Emitter<MemoryType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(&self, memory_type: &MemoryType, output: O) -> Result<usize, CompilerError> {
        self.emit(memory_type.limits(), output)
    }
}

impl Emitter<TableType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        table_type: &TableType,
        mut output: O,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit(table_type.reference_type(), &mut output)?;
        bytes += self.emit(table_type.limits(), &mut output)?;

        Ok(bytes)
    }
}

impl Emitter<GlobalType> for BinaryWebAssemblyEmitter {
    fn emit<O: Write>(
        &self,
        global_type: &GlobalType,
        mut output: O,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit(global_type.value_type(), &mut output)?;

        let mutability: u8 = match global_type.mutability() {
            Mutability::Constant => 0x00,
            Mutability::Variable => 0x01,
        };

        output.write_u8(mutability)?;

        bytes += size_of::<u8>();

        Ok(bytes)
    }
}
