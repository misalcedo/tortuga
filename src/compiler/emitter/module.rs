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
use std::borrow::Borrow;
use std::io::Write;

const PREAMBLE: [u8; 4] = [0x00u8, 0x61u8, 0x73u8, 0x6Du8];
const VERSION: [u8; 4] = [0x01u8, 0x00u8, 0x00u8, 0x00u8];

/// Emit a module to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html
pub fn emit_module<T: Borrow<Module>, O: Write>(
    module: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let mut buffer: Vec<u8> = Vec::new();
    let module = module.borrow();

    bytes += emit_bytes(&PREAMBLE, output, false)?;
    bytes += emit_bytes(&VERSION, output, false)?;
    bytes += emit_version(&mut buffer, output)?;

    if !module.types().is_empty() {
        emit_vector(module.types(), &mut buffer, emit_function_type)?;
        bytes += emit_section(ModuleSection::TypeSection, &mut buffer, output)?;
    }

    if !module.imports().is_empty() {
        emit_vector(module.imports(), &mut buffer, emit_import)?;
        bytes += emit_section(ModuleSection::ImportSection, &mut buffer, output)?;
    }

    if !module.functions().is_empty() {
        let types: Vec<TypeIndex> = module.functions().iter().map(Function::kind).collect();

        emit_vector(types.as_slice(), &mut buffer, emit_usize)?;

        bytes += emit_section(ModuleSection::FunctionSection, &mut buffer, output)?;
    }

    if !module.tables().is_empty() {
        emit_vector(module.tables(), &mut buffer, emit_table)?;
        bytes += emit_section(ModuleSection::TableSection, &mut buffer, output)?;
    }

    if !module.memories().is_empty() {
        emit_vector(module.memories(), &mut buffer, emit_memory)?;
        bytes += emit_section(ModuleSection::MemorySection, &mut buffer, output)?;
    }

    if !module.globals().is_empty() {
        emit_vector(module.globals(), &mut buffer, emit_global)?;
        bytes += emit_section(ModuleSection::GlobalSection, &mut buffer, output)?;
    }

    if !module.exports().is_empty() {
        emit_vector(module.exports(), &mut buffer, emit_export)?;
        bytes += emit_section(ModuleSection::ExportSection, &mut buffer, output)?;
    }

    if let Some(start) = module.start() {
        emit_start(start, &mut buffer)?;
        bytes += emit_section(ModuleSection::StartSection, &mut buffer, output)?;
    }

    if !module.elements().is_empty() {
        emit_vector(module.elements(), &mut buffer, emit_element)?;
        bytes += emit_section(ModuleSection::ElementSection, &mut buffer, output)?;
    }

    if !module.data().is_empty() {
        emit_usize(module.data().len(), &mut buffer)?;
        bytes += emit_section(ModuleSection::DataCountSection, &mut buffer, output)?;
    }

    if !module.functions().is_empty() {
        emit_vector(module.functions(), &mut buffer, emit_function)?;
        bytes += emit_section(ModuleSection::CodeSection, &mut buffer, output)?;
    }

    if !module.data().is_empty() {
        emit_vector(module.data(), &mut buffer, emit_data)?;
        bytes += emit_section(ModuleSection::DataSection, &mut buffer, output)?;
    }

    Ok(bytes)
}

/// Emit a function to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#function-section
fn emit_function<T: Borrow<Function>, O: Write>(
    function: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut bytes = 0;
    let function = function.borrow();

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
pub fn emit_import<T: Borrow<Import>, O: Write>(
    import: Import,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let import = import.borrow();

    bytes += emit_name(import.module(), output)?;
    bytes += emit_name(import.name(), output)?;
    bytes += emit_import_description(import.description(), output)?;

    Ok(bytes)
}

/// Emit an import description to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#import-section
pub fn emit_import_description<T: Borrow<ImportDescription>, O: Write>(
    description: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    match description.borrow() {
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
pub fn emit_table<T: Borrow<Table>, O: Write>(
    table: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_table_type(table.borrow().kind(), output)
}

/// Emit a memory to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#memory-section
pub fn emit_memory<T: Borrow<Memory>, O: Write>(
    memory: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_memory_type(memory.borrow().kind(), output)
}

/// Emit a global to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#global-section
pub fn emit_global<T: Borrow<Global>, O: Write>(
    global: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let value = global.borrow();

    bytes += emit_global_type(value.kind(), output)?;
    bytes += emit_expression(value.initializer(), output)?;

    Ok(bytes)
}

/// Emit an export to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export<T: Borrow<Export>, O: Write>(
    export: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let value = export.borrow();

    bytes += emit_name(value.name(), output)?;
    bytes += emit_export_description(value.description(), output)?;

    Ok(bytes)
}

/// Emit an export description to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#export-section
pub fn emit_export_description<T: Borrow<ExportDescription>, O: Write>(
    description: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let (value, index) = match description.borrow() {
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
pub fn emit_start<T: Borrow<Start>, O: Write>(
    start: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_usize(start.borrow().function(), output)
}

/// Emit an element to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#element-section
pub fn emit_element<T: Borrow<Element>, O: Write>(
    element: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let element = element.borrow();

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
pub fn emit_data<T: Borrow<Data>, O: Write>(
    data: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;
    let data = data.borrow();

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
fn emit_version<O: Write>(buffer: &mut Vec<u8>, output: &mut O) -> Result<usize, CompilerError> {
    let version_section = Name::new("version".to_string());

    emit_custom_content(&version_section, about::VERSION.as_bytes(), buffer)?;
    emit_section(ModuleSection::CustomSection, buffer, output)
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
/// we use a buffer to hold the emitted values and copy the buffer contents to the output.
/// The buffer is reset after it is copied.
fn emit_section<O: Write>(
    section: ModuleSection,
    buffer: &mut Vec<u8>,
    output: &mut O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += emit_module_section(section, output)?;
    bytes += emit_usize(buffer.len(), output)?;
    bytes += output.write(&buffer)?;

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

/// Emit a module section to the output.
///
/// See https://webassembly.github.io/spec/core/binary/modules.html#ModuleSection-section
pub fn emit_module_section<T: Borrow<ModuleSection>, O: Write>(
    section: T,
    output: &mut O,
) -> Result<usize, CompilerError> {
    emit_byte(*section.borrow() as u8, output)
}
