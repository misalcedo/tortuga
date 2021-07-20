use crate::compiler::emitter::BinaryEmitter;
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::Name;
use byteorder::{LittleEndian, WriteBytesExt};
use futures::{AsyncWrite, AsyncWriteExt};
use std::future::Future;

/// See https://webassembly.github.io/spec/core/binary/values.html
impl<'output, O: AsyncWrite + Unpin> BinaryEmitter<'output, O> {
    pub async fn emit_i32(&mut self, value: i32) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        leb128::write::signed(&mut self.value_buffer, value as i64)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_i64(&mut self, value: i64) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        leb128::write::signed(&mut self.value_buffer, value as i64)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_u8(&mut self, value: u8) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        self.value_buffer.write_u8(value)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_u32(&mut self, value: u32) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        leb128::write::unsigned(&mut self.value_buffer, value as u64)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_u64(&mut self, value: u64) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        leb128::write::unsigned(&mut self.value_buffer, value as u64)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_usize(&mut self, value: usize) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        leb128::write::unsigned(&mut self.value_buffer, value as u64)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_f32(&mut self, value: f32) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        self.value_buffer.write_f32::<LittleEndian>(value)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_f64(&mut self, value: f64) -> Result<usize, CompilerError> {
        self.value_buffer.clear();
        self.value_buffer.write_f64::<LittleEndian>(value)?;

        self.output.write_all(&self.value_buffer).await?;
        Ok(self.value_buffer.len())
    }

    pub async fn emit_name(&mut self, value: &Name) -> Result<usize, CompilerError> {
        self.emit_byte_vector(value.as_bytes()).await
    }

    pub async fn emit_bytes(&mut self, value: &[u8]) -> Result<usize, CompilerError> {
        self.output.write_all(value).await?;

        Ok(value.len())
    }

    pub async fn emit_byte_vector(&mut self, value: &[u8]) -> Result<usize, CompilerError> {
        self.emit_vector(value, self.emit_u8).await
    }

    pub async fn emit_vector<T, F, E>(
        &mut self,
        values: &[T],
        emitter: E,
    ) -> Result<usize, CompilerError>
    where
        F: Future<Output = Result<usize, CompilerError>>,
        E: Fn(T) -> F,
    {
        let mut bytes = 0;

        bytes += self.emit_usize(values.len()).await?;

        for value in values {
            bytes += emitter(value).await?;
        }

        Ok(bytes)
    }
}
