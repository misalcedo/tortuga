use crate::compiler::emitter::Emit;
use crate::compiler::errors::{CompilerError, ErrorKind};
use crate::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementKind, ElementMode, Export,
    ExportDescription, Function, Global, Import, ImportDescription, Memory, Module, Name, Start,
    Table, TypeIndex,
};
use std::io::{Cursor, Write};

const PREAMBLE: &[u8; 4] = b"\x00\x61\x73\x6D";
const VERSION: &[u8; 4] = b"\x01\x00\x00\x00";

/// See https://webassembly.github.io/spec/core/binary/modules.html
impl Emit for Module {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;
        let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        bytes += output.write(PREAMBLE)?;
        bytes += output.write(VERSION)?;

        if !self.types().is_empty() {
            self.types().emit(&mut buffer)?;
            emit_section(ModuleSection::TypeSection, &mut buffer, output)?;
        }

        if !self.imports().is_empty() {
            self.imports().emit(&mut buffer)?;
            emit_section(ModuleSection::ImportSection, &mut buffer, output)?;
        }

        if !self.functions().is_empty() {
            let types: Vec<TypeIndex> = self.functions().iter().map(Function::type_index).collect();

            types.as_slice().emit(&mut buffer)?;

            emit_section(ModuleSection::FunctionSection, &mut buffer, output)?;
        }

        if !self.tables().is_empty() {
            self.tables().emit(&mut buffer)?;
            emit_section(ModuleSection::TableSection, &mut buffer, output)?;
        }

        if !self.memories().is_empty() {
            self.memories().emit(&mut buffer)?;
            emit_section(ModuleSection::MemorySection, &mut buffer, output)?;
        }

        if !self.globals().is_empty() {
            self.globals().emit(&mut buffer)?;
            emit_section(ModuleSection::GlobalSection, &mut buffer, output)?;
        }

        if !self.exports().is_empty() {
            self.exports().emit(&mut buffer)?;
            emit_section(ModuleSection::ExportSection, &mut buffer, output)?;
        }

        if let Some(start) = self.start() {
            start.emit(&mut buffer)?;
            emit_section(ModuleSection::StartSection, &mut buffer, output)?;
        }

        if !self.elements().is_empty() {
            self.elements().emit(&mut buffer)?;
            emit_section(ModuleSection::ElementSection, &mut buffer, output)?;
        }

        if !self.data().is_empty() {
            self.data().len().emit(&mut buffer)?;
            emit_section(ModuleSection::DataCountSection, &mut buffer, output)?;
        }

        if !self.functions().is_empty() {
            emit_section(ModuleSection::CodeSection, &mut buffer, output)?;
        }

        if !self.data().is_empty() {
            self.data().emit(&mut buffer)?;
            emit_section(ModuleSection::DataSection, &mut buffer, output)?;
        }

        Ok(bytes)
    }
}

impl Emit for Import {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.module().emit(output)?;
        bytes += self.name().emit(output)?;
        bytes += self.description().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for ImportDescription {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self {
            ImportDescription::Function(index) => {
                bytes += 0x00u8.emit(output)?;
                bytes += index.emit(output)?;
            }
            ImportDescription::Table(table_type) => {
                bytes += 0x01u8.emit(output)?;
                bytes += table_type.emit(output)?;
            }
            ImportDescription::Memory(memory_type) => {
                bytes += 0x02u8.emit(output)?;
                bytes += memory_type.emit(output)?;
            }
            ImportDescription::Global(global_type) => {
                bytes += 0x03u8.emit(output)?;
                bytes += global_type.emit(output)?;
            }
        };

        Ok(bytes)
    }
}

impl Emit for Table {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.kind().emit(output)
    }
}

impl Emit for Memory {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.memory_type().emit(output)
    }
}

impl Emit for Global {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.kind().emit(output)?;
        bytes += self.initializer().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for Export {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        bytes += self.name().emit(output)?;
        bytes += self.description().emit(output)?;

        Ok(bytes)
    }
}

impl Emit for ExportDescription {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let (value, index) = match self {
            ExportDescription::Function(index) => (0x00, index),
            ExportDescription::Table(index) => (0x01, index),
            ExportDescription::Memory(index) => (0x02, index),
            ExportDescription::Global(index) => (0x03, index),
        };
        let mut bytes = 0;

        bytes += value.emit(output)?;
        bytes += index.emit(output)?;

        Ok(bytes)
    }
}

impl Emit for Start {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        self.function().emit(output)
    }
}

impl Emit for Element {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match (self.initializers(), self.mode(), self.kind()) {
            (
                ElementInitializer::FunctionIndex(indices),
                ElementMode::Active(0, offset),
                ElementKind::FunctionReference,
            ) => {
                bytes += 0x00u8.emit(output)?;
                bytes += offset.emit(output)?;
                bytes += indices.emit(output)?;
            }
            (ElementInitializer::FunctionIndex(indices), ElementMode::Passive, kind) => {
                bytes += 0x01u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += indices.emit(output)?;
            }
            (
                ElementInitializer::FunctionIndex(indices),
                ElementMode::Active(table, offset),
                kind,
            ) => {
                bytes += 0x02u8.emit(output)?;
                bytes += table.emit(output)?;
                bytes += offset.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += indices.emit(output)?;
            }
            (ElementInitializer::FunctionIndex(indices), ElementMode::Declarative, kind) => {
                bytes += 0x03u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += indices.emit(output)?;
            }
            (
                ElementInitializer::Expression(expressions),
                ElementMode::Active(0, offset),
                ElementKind::FunctionReference,
            ) => {
                bytes += 0x04u8.emit(output)?;
                bytes += offset.emit(output)?;
                bytes += expressions.emit(output)?;
            }
            (
                ElementInitializer::Expression(expressions),
                ElementMode::Passive,
                ElementKind::ReferenceType(kind),
            ) => {
                bytes += 0x05u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expressions.emit(output)?;
            }
            (
                ElementInitializer::Expression(expressions),
                ElementMode::Active(table, offset),
                ElementKind::ReferenceType(kind),
            ) => {
                bytes += 0x06u8.emit(output)?;
                bytes += table.emit(output)?;
                bytes += offset.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expressions.emit(output)?;
            }
            (
                ElementInitializer::Expression(expressions),
                ElementMode::Declarative,
                ElementKind::ReferenceType(kind),
            ) => {
                bytes += 0x07u8.emit(output)?;
                bytes += kind.emit(output)?;
                bytes += expressions.emit(output)?;
            }
            _ => return Err(ErrorKind::InvalidSyntax.into()),
        };

        Ok(bytes)
    }
}

impl Emit for ElementKind {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        match self {
            ElementKind::FunctionReference => 0x00u8.emit(output),
            ElementKind::ReferenceType(kind) => kind.emit(output),
        }
    }
}

impl Emit for Data {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        let mut bytes = 0;

        match self.mode() {
            DataMode::Active(0, offset) => {
                bytes += 0x00u8.emit(output)?;
                bytes += offset.emit(output)?;
            }
            DataMode::Passive => {
                bytes += 0x01u8.emit(output)?;
            }
            DataMode::Active(memory, offset) => {
                bytes += 0x02u8.emit(output)?;
                bytes += memory.emit(output)?;
                bytes += offset.emit(output)?;
            }
        };

        bytes += self.len().emit(output)?;
        bytes += self.initializer().emit(output)?;

        Ok(bytes)
    }
}

fn emit_custom_section<O: Write>(
    name: &Name,
    content: &[u8],
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += name.emit(output)?;
    bytes += content.emit(output)?;

    Ok(bytes)
}

/// Emits a module section to the given output.
/// Sections need to be prefixed by their length.
/// Since we do not know the length of the emitted contents ahead of time,
/// we use a buffer to hold the emitted values and copy the buffer contents to the output.
/// The buffer is reset after it is copied.
fn emit_section<O: Write>(
    section: ModuleSection,
    buffer: &mut Cursor<Vec<u8>>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += section.emit(output)?;
    bytes += buffer.position().emit(output)?;
    bytes += std::io::copy(buffer, output)? as usize;

    buffer.set_position(0);

    Ok(bytes)
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

impl Emit for ModuleSection {
    fn emit<O: Write>(&self, output: &mut O) -> Result<usize, CompilerError> {
        (*self as u8).emit(output)
    }
}
