use crate::about;
use crate::compiler::emitter::instruction::emit_expression;
use crate::compiler::emitter::{
    emit_byte, emit_bytes, emit_function_type, emit_global_type, emit_i32, emit_memory_type,
    emit_name, emit_reference_type, emit_table_type, emit_u32, emit_usize, emit_value_type,
    emit_vector,
};
use crate::compiler::errors::CompilerError;
use crate::syntax::web_assembly::{
    Data, DataMode, Element, ElementInitializer, ElementMode, Export, ExportDescription, Function,
    Global, Import, ImportDescription, Memory, Module, Name, ReferenceType, Start, Table,
    TypeIndex,
};
use futures::{AsyncWrite, AsyncWriteExt};
use std::io::Write;

/// A magic constant used to quickly identify WebAssembly binary file contents.
const PREAMBLE: [u8; 4] = [0x00u8, 0x61u8, 0x73u8, 0x6Du8];

/// The version of the binary WebAssembly format emitted.
const VERSION: [u8; 4] = [0x01u8, 0x00u8, 0x00u8, 0x00u8];

/// Emit a module to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html
pub async fn emit_module<O: AsyncWrite + Unpin>(
    module: &Module,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let mut section_buffer: Vec<u8> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();

    bytes += emit_bytes(&PREAMBLE, &mut buffer, false)?;
    bytes += emit_bytes(&VERSION, &mut buffer, false)?;
    bytes += emit_version_custom_section(&mut section_buffer, &mut buffer)?;
    bytes += emit_type_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_import_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_function_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_table_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_memory_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_global_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_export_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_start_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_element_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_data_count_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_code_section(module, &mut section_buffer, &mut buffer)?;
    bytes += emit_data_section(module, &mut section_buffer, &mut buffer)?;

    output.write_all(&buffer[..]).await?;

    Ok(bytes)
}

/// Emits the type section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#type-section
fn emit_type_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.types().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::TypeSection, buffer, output, |o| {
            emit_vector(module.types(), o, emit_function_type)
        })
    }
}

/// Emits the import section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
fn emit_import_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.imports().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::ImportSection, buffer, output, |o| {
            emit_vector(module.imports(), o, emit_import)
        })
    }
}

/// Emits the function section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#function-section
fn emit_function_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.functions().is_empty() {
        Ok(0)
    } else {
        let types: Vec<TypeIndex> = module.functions().iter().map(Function::kind).collect();

        emit_section(ModuleSection::FunctionSection, buffer, output, move |o| {
            emit_vector(types.as_slice(), o, emit_usize)
        })
    }
}

/// Emits the table section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#table-section
fn emit_table_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.tables().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::TableSection, buffer, output, |o| {
            emit_vector(module.tables(), o, emit_table)
        })
    }
}

/// Emits the memory section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#memory-section
fn emit_memory_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.memories().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::MemorySection, buffer, output, |o| {
            emit_vector(module.memories(), o, emit_memory)
        })
    }
}

/// Emits the global section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#global-section
fn emit_global_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.globals().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::GlobalSection, buffer, output, |o| {
            emit_vector(module.globals(), o, emit_global)
        })
    }
}

/// Emits the export section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
fn emit_export_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.exports().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::ExportSection, buffer, output, |o| {
            emit_vector(module.exports(), o, emit_export)
        })
    }
}
/// Emits the start section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#start-section
fn emit_start_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    match module.start() {
        Some(start) => emit_section(ModuleSection::StartSection, buffer, output, |o| {
            emit_start(start, o)
        }),
        None => Ok(0),
    }
}
/// Emits the elements section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#elements-section
fn emit_element_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.elements().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::ElementSection, buffer, output, |o| {
            emit_vector(module.elements(), o, emit_element)
        })
    }
}

/// Emits the data count section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#data-count-section
fn emit_data_count_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.data().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::DataCountSection, buffer, output, |o| {
            emit_usize(module.data().len(), o)
        })
    }
}

/// Emits the code section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#code-section
fn emit_code_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.functions().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::CodeSection, buffer, output, |o| {
            emit_vector(module.functions(), o, emit_function)
        })
    }
}

/// Emits the data section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#data-section
fn emit_data_section<O: Write>(
    module: &Module,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    if module.data().is_empty() {
        Ok(0)
    } else {
        emit_section(ModuleSection::DataSection, buffer, output, |o| {
            emit_vector(module.data(), o, emit_data)
        })
    }
}

/// Emit a function to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#function-section
fn emit_function<O: Write>(function: &Function, output: &mut O) -> Result<usize, CompilerError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut bytes = 0;

    emit_usize(function.locals().len(), &mut buffer)?;
    for local in function.locals().kinds() {
        emit_u32(1u32, &mut buffer)?;
        emit_value_type(local, &mut buffer)?;
    }

    emit_expression(function.body(), &mut buffer)?;

    bytes += emit_usize(buffer.len(), output)?;
    bytes += output.write(&buffer)?;

    Ok(bytes)
}

/// Emit an import to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
pub fn emit_import<O: Write>(import: &Import, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_name(import.module(), output)?;
    bytes += emit_name(import.name(), output)?;
    bytes += emit_import_description(import.description(), output)?;

    Ok(bytes)
}

/// Emit an import description to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
pub fn emit_import_description<O: Write>(
    description: &ImportDescription,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match description {
        ImportDescription::Function(index) => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_usize(index, output)?;
        }
        ImportDescription::Table(table_type) => {
            bytes += emit_byte(0x01u8, output)?;
            bytes += emit_table_type(table_type, output)?;
        }
        ImportDescription::Memory(memory_type) => {
            bytes += emit_byte(0x02u8, output)?;
            bytes += emit_memory_type(memory_type, output)?;
        }
        ImportDescription::Global(global_type) => {
            bytes += emit_byte(0x03u8, output)?;
            bytes += emit_global_type(global_type, output)?;
        }
    };

    Ok(bytes)
}

/// Emit a table to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#table-section
pub fn emit_table<O: Write>(table: &Table, output: &mut O) -> Result<usize, CompilerError> {
    emit_table_type(table.kind(), output)
}

/// Emit a memory to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub fn emit_memory<O: Write>(memory: &Memory, output: &mut O) -> Result<usize, CompilerError> {
    emit_memory_type(memory.kind(), output)
}

/// Emit a global to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#global-section
pub fn emit_global<O: Write>(global: &Global, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_global_type(global.kind(), output)?;
    bytes += emit_expression(global.initializer(), output)?;

    Ok(bytes)
}

/// Emit an export to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export<O: Write>(export: &Export, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_name(export.name(), output)?;
    bytes += emit_export_description(export.description(), output)?;

    Ok(bytes)
}

/// Emit an export description to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export_description<O: Write>(
    description: &ExportDescription,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let (value, index) = match description {
        ExportDescription::Function(index) => (0x00, index),
        ExportDescription::Table(index) => (0x01, index),
        ExportDescription::Memory(index) => (0x02, index),
        ExportDescription::Global(index) => (0x03, index),
    };
    let mut bytes = 0;

    bytes += emit_i32(value, output)?;
    bytes += emit_usize(index, output)?;

    Ok(bytes)
}

/// Emit a start to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#start-section
pub fn emit_start<O: Write>(start: &Start, output: &mut O) -> Result<usize, CompilerError> {
    emit_usize(start.function(), output)
}

/// Emit an element to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#element-section
pub fn emit_element<O: Write>(element: &Element, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match (element.initializers(), element.mode(), element.kind()) {
        (
            ElementInitializer::FunctionIndex(indices),
            ElementMode::Active(0, offset),
            ReferenceType::Function,
        ) => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (
            ElementInitializer::FunctionIndex(indices),
            ElementMode::Passive,
            ReferenceType::Function,
        ) => {
            bytes += emit_byte(0x01u8, output)?;
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (ElementInitializer::FunctionIndex(indices), ElementMode::Active(table, offset), kind) => {
            bytes += emit_byte(0x02u8, output)?;
            bytes += emit_usize(table, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (ElementInitializer::FunctionIndex(indices), ElementMode::Declarative, kind) => {
            bytes += emit_byte(0x03u8, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(indices, output, emit_usize)?;
        }
        (
            ElementInitializer::Expression(expressions),
            ElementMode::Active(0, offset),
            ReferenceType::Function,
        ) => {
            bytes += emit_byte(0x04u8, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        (ElementInitializer::Expression(expressions), ElementMode::Passive, kind) => {
            bytes += emit_byte(0x05u8, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        (ElementInitializer::Expression(expressions), ElementMode::Active(table, offset), kind) => {
            bytes += emit_byte(0x06u8, output)?;
            bytes += emit_usize(table, output)?;
            bytes += emit_expression(offset, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        (ElementInitializer::Expression(expressions), ElementMode::Declarative, kind) => {
            bytes += emit_byte(0x07u8, output)?;
            bytes += emit_reference_type(kind, output)?;
            bytes += emit_vector(expressions, output, emit_expression)?;
        }
        _ => return Err(CompilerError::InvalidSyntax),
    };

    Ok(bytes)
}

/// Emit a data to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#data-section
pub fn emit_data<O: Write>(data: &Data, output: &mut O) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match data.mode() {
        DataMode::Active(0, offset) => {
            bytes += emit_byte(0x00u8, output)?;
            bytes += emit_expression(offset, output)?;
        }
        DataMode::Passive => {
            bytes += emit_byte(0x01u8, output)?;
        }
        DataMode::Active(memory, offset) => {
            bytes += emit_byte(0x02u8, output)?;
            bytes += emit_usize(memory, output)?;
            bytes += emit_expression(offset, output)?;
        }
    };

    bytes += emit_bytes(data.initializer(), output, true)?;

    Ok(bytes)
}

/// Emit a custom section with the version of the language the module was compiled.
fn emit_version_custom_section<O: Write>(
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_section(ModuleSection::CustomSection, buffer, output, |o| {
        let version_section = Name::new("version".to_string());
        emit_custom_content(&version_section, about::VERSION.as_bytes(), o)
    })
}

/// Emit named custom content to the module.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#custom-section
fn emit_custom_content<O: Write>(
    name: &Name,
    content: &[u8],
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_name(name, output)?;
    bytes += emit_bytes(content, output, false)?;

    Ok(bytes)
}

/// Emits a module section to the given output.
/// Sections need to be prefixed by their length.
/// Since we do not know the length of the emitted contents ahead of time,
/// a buffer is used to hold the emitted values and copy the buffer contents to the output.
/// The buffer is reset before emitting content and after it is copied.
fn emit_section<E, O>(
    section: ModuleSection,
    buffer: &mut Vec<u8>,
    output: &mut O,
    emit: E,
) -> Result<usize, CompilerError>
where
    O: Write,
    E: Fn(&mut Vec<u8>) -> Result<usize, CompilerError>,
{
    buffer.clear();

    let mut bytes = 0;

    emit(buffer)?;

    bytes += emit_byte(section as u8, output)?;
    bytes += emit_usize(buffer.len(), output)?;
    bytes += buffer.len();

    output.write_all(&buffer)?;
    buffer.clear();

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
