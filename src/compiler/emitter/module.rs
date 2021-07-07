use crate::compiler::emitter::Emit;
use crate::compiler::errors::CompilerError;
use crate::web_assembly::{Module, Name};
use byteorder::WriteBytesExt;
use std::io::{Cursor, Write};
use std::mem::size_of;

const PREAMBLE: &[u8; 4] = b"\x00\x61\x73\x6D";
const VERSION: &[u8; 4] = b"\x01\x00\x00\x00";

/// See https://webassembly.github.io/spec/core/binary/modules.html
impl Emit for Module {
    fn emit<O: Write>(&self, mut output: O) -> Result<usize, CompilerError> {
        let mut bytes = 0;
        let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        bytes += output.write(PREAMBLE)?;
        bytes += output.write(VERSION)?;

        Ok(bytes)
    }
}

fn emit_custom_section<O: Write>(
    name: &Name,
    content: &[u8],
    mut output: O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += name.emit(&mut output)?;
    bytes += content.emit(&mut output)?;

    Ok(bytes)
}

fn emit_section<O: Write>(
    section: ModuleSection,
    mut buffer: &mut Cursor<Vec<u8>>,
    mut output: O,
) -> Result<usize, CompilerError> {
    let mut bytes = 0;

    bytes += section.emit(&mut output)?;
    bytes += buffer.position().emit(&mut output)?;
    bytes += std::io::copy(buffer, &mut output)? as usize;

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
    fn emit<O: Write>(&self, mut output: O) -> Result<usize, CompilerError> {
        output.write_u8(*self as u8)?;

        Ok(size_of::<u8>())
    }
}
