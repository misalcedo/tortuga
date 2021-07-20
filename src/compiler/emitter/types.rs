use crate::compiler::emitter::BinaryEmitter;
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{
    FunctionType, GlobalType, Limit, MemoryType, NumberType, ReferenceType, ResultType, TableType,
    ValueType,
};
use futures::AsyncWrite;

impl<'output, O: AsyncWrite + Unpin> BinaryEmitter<'output, O> {
    pub async fn emit_number_type(&mut self, value: &NumberType) -> Result<usize, CompilerError> {
        let output: u8 = match value {
            NumberType::I32 => 0x7F,
            NumberType::I64 => 0x7E,
            NumberType::F32 => 0x7D,
            NumberType::F64 => 0x7C,
        };

        self.emit_u8(output).await
    }

    pub async fn emit_reference_type(
        &mut self,
        value: &ReferenceType,
    ) -> Result<usize, CompilerError> {
        let output: u8 = match value {
            ReferenceType::Function => 0x70,
            ReferenceType::External => 0x6F,
        };

        self.emit_u8(output).await
    }

    pub async fn emit_value_type(&mut self, value: &ValueType) -> Result<usize, CompilerError> {
        match value {
            ValueType::Number(number_type) => self.emit_number_type(number_type).await,
            ValueType::Reference(reference_type) => self.emit_reference_type(reference_type).await,
        }
    }

    pub async fn emit_result_type(&mut self, value: &ResultType) -> Result<usize, CompilerError> {
        self.emit_vector(value.kinds(), Self::emit_value_type).await
    }

    pub async fn emit_function_type(
        &mut self,
        value: &FunctionType,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_u8(0x60).await?;
        bytes += self.emit_result_type(value.parameters()).await?;
        bytes += self.emit_result_type(value.results()).await?;

        Ok(bytes)
    }

    pub async fn emit_limit(&mut self, value: &Limit) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match value.max() {
            None => {
                bytes += self.emit_u8(0x00).await?;
                bytes += self.emit_usize(value.min()).await?;
            }
            Some(max) => {
                bytes += self.emit_u8(0x01).await?;
                bytes += self.emit_usize(value.min()).await?;
                bytes += self.emit_usize(max).await?;
            }
        };

        Ok(bytes)
    }

    pub async fn emit_table_type(&mut self, value: &TableType) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_reference_type(value.kind()).await?;
        bytes += self.emit_limit(value.limits()).await?;

        Ok(bytes)
    }

    pub async fn emit_memory_type(&mut self, value: &MemoryType) -> Result<usize, CompilerError> {
        self.emit_limit(value.limits()).await
    }

    pub async fn emit_global_type(&mut self, value: &GlobalType) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_value_type(value.kind()).await?;

        let mutability: u8 = match value.is_mutable() {
            false => 0x00,
            true => 0x01,
        };

        bytes += self.emit_u8(mutability).await?;

        Ok(bytes)
    }
}
