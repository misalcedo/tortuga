use crate::about;
use crate::compiler::emitter::BinaryEmitter;
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription, Function,
    Global, Import, ImportDescription, Memory, Module, Name, ReferenceType, Start, Table,
    TypeIndex,
};
use futures::{AsyncWrite, AsyncWriteExt};

const PREAMBLE: [u8; 4] = [0x00u8, 0x61u8, 0x73u8, 0x6Du8];
const VERSION: [u8; 4] = [0x01u8, 0x00u8, 0x00u8, 0x00u8];

impl<'output, O: AsyncWrite + Unpin> BinaryEmitter<'output, O> {
    /// Emit named custom content to the module.
    async fn emit_custom_content(
        &mut self,
        name: &Name,
        content: &[u8],
    ) -> Result<usize, CompilerError> {
        self.emit_name(name).await?;
        self.emit_bytes(content).await?;

        self.emit_section(ModuleSection::CustomSection).await
    }

    /// Emit a custom section with the version of the language the module was compiled.
    async fn emit_version(&mut self) -> Result<usize, CompilerError> {
        let version_section = Name::new("version".to_string());

        self.emit_custom_content(&version_section, about::VERSION.as_bytes())
            .await
    }

    /// Emits a module section to the given output.
    /// Sections need to be prefixed by their length.
    /// Since we do not know the length of the emitted contents ahead of time,
    /// we use a buffer to hold the emitted values and copy the buffer contents to the output.
    /// The buffer is reset after it is copied.
    async fn emit_section(&mut self, section: ModuleSection) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_u8(section as u8).await?;
        bytes += self.emit_usize(self.section_buffer.len()).await?;

        self.output.write_all(&self.section_buffer).await?;

        bytes += self.section_buffer.len();

        self.section_buffer.clear();

        Ok(bytes)
    }

    /// See https://webassembly.github.io/spec/core/binary/modules.html
    pub async fn emit_module(&mut self, module: &Module) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_bytes(&PREAMBLE).await?;
        bytes += self.emit_bytes(&VERSION).await?;
        bytes += self.emit_version().await?;

        if !module.types().is_empty() {
            module.types().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::TypeSection).await?;
        }

        if !module.imports().is_empty() {
            module.imports().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::ImportSection).await?;
        }

        if !module.functions().is_empty() {
            let types: Vec<TypeIndex> = module.functions().iter().map(Function::kind).collect();

            types.as_slice().emit(&mut self.section_buffer)?;

            bytes += self.emit_section(ModuleSection::FunctionSection).await?;
        }

        if !module.tables().is_empty() {
            module.tables().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::TableSection).await?;
        }

        if !module.memories().is_empty() {
            module.memories().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::MemorySection).await?;
        }

        if !module.globals().is_empty() {
            module.globals().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::GlobalSection).await?;
        }

        if !module.exports().is_empty() {
            module.exports().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::ExportSection).await?;
        }

        if let Some(start) = module.start() {
            start.emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::StartSection).await?;
        }

        if !module.elements().is_empty() {
            module.elements().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::ElementSection).await?;
        }

        if !module.data().is_empty() {
            module.data().len().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::DataCountSection).await?;
        }

        if !module.functions().is_empty() {
            module.functions().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::CodeSection).await?;
        }

        if !module.data().is_empty() {
            module.data().emit(&mut self.section_buffer)?;
            bytes += self.emit_section(ModuleSection::DataSection).await?;
        }

        Ok(bytes)
    }

    pub async fn emit_function(&mut self, value: &Function) -> Result<usize, CompilerError> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut temp = BinaryEmitter::new(&mut buffer);
        let mut bytes = 0;

        temp.emit_usize(value.locals().len()).await?;
        for local in value.locals().kinds() {
            temp.emit_u32(1).await?;
            temp.emit_value_type(local).await?;
        }

        temp.emit_expression(value.body()).await?;

        bytes += self.emit_usize(buffer.len()).await?;
        self.output.write_all(&buffer).await?;
        bytes += buffer.len();

        Ok(bytes)
    }

    pub async fn emit_table(&mut self, value: &Table) -> Result<usize, CompilerError> {
        self.emit_table_type(value.kind()).await
    }

    pub async fn emit_memory(&mut self, value: &Memory) -> Result<usize, CompilerError> {
        self.emit_memory_type(value.kind()).await
    }

    pub async fn emit_global(&mut self, value: &Global) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_global_type(value.kind()).await?;
        bytes += self.emit_expression(value.initializer()).await?;

        Ok(bytes)
    }

    pub async fn emit_import(&mut self, value: &Import) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_name(value.module()).await?;
        bytes += self.emit_name(value.name()).await?;
        bytes += self.emit_import_description(value.description()).await?;

        Ok(bytes)
    }

    pub async fn emit_import_description(
        &mut self,
        value: &ImportDescription,
    ) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match value {
            ImportDescription::Function(index) => {
                bytes += self.emit_u8(0x00).await?;
                bytes += self.emit_usize(*index).await?;
            }
            ImportDescription::Table(table_type) => {
                bytes += self.emit_u8(0x01).await?;
                bytes += self.emit_table_type(table_type).await?;
            }
            ImportDescription::Memory(memory_type) => {
                bytes += self.emit_u8(0x02).await?;
                bytes += self.emit_memory_type(memory_type).await?;
            }
            ImportDescription::Global(global_type) => {
                bytes += self.emit_u8(0x03).await?;
                bytes += self.emit_global_type(global_type).await?;
            }
        };

        Ok(bytes)
    }

    pub async fn emit_export(&mut self, value: &Export) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.emit_name(value.name()).await?;
        bytes += self.emit_export_description(value.description()).await?;

        Ok(bytes)
    }

    pub async fn emit_export_description(
        &mut self,
        value: &ExportDescription,
    ) -> Result<usize, CompilerError> {
        let (value, index) = match value {
            ExportDescription::Function(index) => (0x00, index),
            ExportDescription::Table(index) => (0x01, index),
            ExportDescription::Memory(index) => (0x02, index),
            ExportDescription::Global(index) => (0x03, index),
        };
        let mut bytes = 0;

        bytes += self.emit_usize(value).await?;
        bytes += self.emit_usize(*index).await?;

        Ok(bytes)
    }

    pub async fn emit_start(&mut self, value: &Start) -> Result<usize, CompilerError> {
        self.emit_usize(value.function()).await
    }

    pub async fn emit_element(&mut self, value: &Element) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match (value.initializers(), value.mode(), value.kind()) {
            (
                ElementInitializer::FunctionIndex(indices),
                ElementMode::Active(0, offset),
                ReferenceType::Function,
            ) => {
                bytes += self.emit_u8(0x00).await?;
                bytes += self.emit_expression(&offset).await?;
                bytes += self.emit_vector(indices, self.emit_usize).await?;
            }
            (
                ElementInitializer::FunctionIndex(indices),
                ElementMode::Passive,
                ReferenceType::Function,
            ) => {
                bytes += self.emit_u8(0x01).await?;
                bytes += self.emit_u8(0x00).await?;
                bytes += self.emit_vector(indices, self.emit_usize).await?;
            }
            (
                ElementInitializer::FunctionIndex(indices),
                ElementMode::Active(table, offset),
                kind,
            ) => {
                bytes += self.emit_u8(0x02).await?;
                bytes += self.emit_usize(*table).await?;
                bytes += self.emit_expression(offset).await?;
                bytes += self.emit_reference_type(kind).await?;
                bytes += self.emit_vector(indices, self.emit_usize).await?;
            }
            (ElementInitializer::FunctionIndex(indices), ElementMode::Declarative, kind) => {
                bytes += self.emit_u8(0x03).await?;
                bytes += self.emit_reference_type(kind).await?;
                bytes += self.emit_vector(indices, self.emit_usize).await?;
            }
            (
                ElementInitializer::Expression(expressions),
                ElementMode::Active(0, offset),
                ReferenceType::Function,
            ) => {
                bytes += self.emit_u8(0x04).await?;
                bytes += self.emit_expression(offset).await?;
                bytes += self.emit_vector(expressions, self.emit_expression).await?;
            }
            (ElementInitializer::Expression(expressions), ElementMode::Passive, kind) => {
                bytes += self.emit_u8(0x05).await?;
                bytes += self.emit_reference_type(kind).await?;
                bytes += self.emit_vector(expressions, self.emit_expression).await?;
            }
            (
                ElementInitializer::Expression(expressions),
                ElementMode::Active(table, offset),
                kind,
            ) => {
                bytes += self.emit_u8(0x06).await?;
                bytes += self.emit_usize(*table).await?;
                bytes += self.emit_expression(offset).await?;
                bytes += self.emit_reference_type(kind).await?;
                bytes += self.emit_vector(expressions, self.emit_expression).await?;
            }
            (ElementInitializer::Expression(expressions), ElementMode::Declarative, kind) => {
                bytes += self.emit_u8(0x07).await?;
                bytes += self.emit_reference_type(kind).await?;
                bytes += self.emit_vector(expressions, self.emit_expression).await?;
            }
            _ => return Err(CompilerError::InvalidSyntax),
        };

        Ok(bytes)
    }

    pub async fn emit_data(&mut self, value: &Data) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match value.mode() {
            DataMode::Active(0, offset) => {
                bytes += self.emit_u8(0x00).await?;
                bytes += self.emit_expression(offset).await?;
            }
            DataMode::Passive => {
                bytes += self.emit_u8(0x01).await?;
            }
            DataMode::Active(memory, offset) => {
                bytes += self.emit_u8(0x02).await?;
                bytes += self.emit_usize(*memory).await?;
                bytes += self.emit_expression(offset).await?;
            }
        };

        bytes += self.emit_vector(value.initializer(), self.emit_u8).await?;

        Ok(bytes)
    }
}

#[derive(Copy, Clone)]
pub enum ModuleSection {
    /// Custom sections have the id 0.
    /// They are intended to be used for debugging information or third-party extensions,
    /// and are ignored by the WebAssembly semantics.
    /// Their contents consist of a name further identifying the custom section,
    /// followed by an uninterpreted sequence of bytes for custom use.
    CustomSection = 0,
    /// The type section has the id 1.
    /// It decodes into a vector of function types that represent the 𝗍𝗒𝗉𝖾𝗌 component of a module.
    TypeSection,
    /// The import section has the id 2.
    /// It decodes into a vector of imports that represent the 𝗂𝗆𝗉𝗈𝗋𝗍𝗌 component of a module.
    ImportSection,
    /// The function section has the id 3.
    /// It decodes into a vector of type indices that represent the 𝗍𝗒𝗉𝖾 fields of the functions
    /// in the 𝖿𝗎𝗇𝖼𝗌 component of a module. The 𝗅𝗈𝖼𝖺𝗅𝗌 and 𝖻𝗈𝖽𝗒 fields of the respective functions
    /// are encoded separately in the code section.
    FunctionSection,
    /// The table section has the id 4.
    /// It decodes into a vector of tables that represent the 𝗍𝖺𝖻𝗅𝖾𝗌 component of a module.
    TableSection,
    /// The memory section has the id 5.
    /// It decodes into a vector of memories that represent the 𝗆𝖾𝗆𝗌 component of a module.
    MemorySection,
    /// The global section has the id 6.
    /// It decodes into a vector of globals that represent the 𝗀𝗅𝗈𝖻𝖺𝗅𝗌 component of a module.
    GlobalSection,
    /// The export section has the id 7.
    /// It decodes into a vector of exports that represent the 𝖾𝗑𝗉𝗈𝗋𝗍𝗌 component of a module.
    ExportSection,
    /// The start section has the id 8.
    /// It decodes into an optional start function that represents the 𝗌𝗍𝖺𝗋𝗍 component of a module.
    StartSection,
    /// The element section has the id 9.
    /// It decodes into a vector of element segments that represent the 𝖾𝗅𝖾𝗆𝗌 component of a module.
    ElementSection,
    /// The code section has the id 10.
    /// It decodes into a vector of code entries that are pairs of value type vectors and expressions.
    /// They represent the 𝗅𝗈𝖼𝖺𝗅𝗌 and 𝖻𝗈𝖽𝗒 field of the functions in the 𝖿𝗎𝗇𝖼𝗌 component of a module.
    /// The 𝗍𝗒𝗉𝖾 fields of the respective functions are encoded separately in the function section.
    CodeSection,
    /// The data section has the id 11.
    /// It decodes into a vector of data segments that represent the 𝖽𝖺𝗍𝖺𝗌 component of a module.
    DataSection,
    /// The data count section has the id 12.
    /// It decodes into an optional u32 that represents the number of data segments in the data section.
    /// If this count does not match the length of the data segment vector, the module is malformed.
    DataCountSection,
}
