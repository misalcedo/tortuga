use crate::web_assembly::types::*;
use crate::web_assembly::{Expression, Name};

pub struct Module {
    types: Vec<FunctionType>,
    functions: Vec<Function>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    elements: Vec<Element>,
    data: Vec<Data>,
    start: Option<Start>,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            types: Vec::new(),
            functions: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            elements: Vec::new(),
            data: Vec::new(),
            start: None,
            imports: Vec::new(),
            exports: Vec::new(),
        }
    }
}

pub struct TypeIndex(usize);
pub struct FunctionIndex(usize);
pub struct TableIndex(usize);
pub struct MemoryIndex(usize);
pub struct GlobalIndex(usize);
pub struct ElementIndex(usize);
pub struct DataIndex(usize);
pub struct LocalIndex(usize);
pub struct LabelIndex(usize);

/// The 𝗍𝗒𝗉𝖾 of a function declares its signature by reference to a type defined in the module.
/// The parameters of the function are referenced through 0-based local indices in the function’s
/// body;they are mutable.
/// The 𝗅𝗈𝖼𝖺𝗅𝗌 declare a vector of mutable local variables and their types.
/// These variables are referenced through local indices in the function’s body.
/// The index of the first local is the smallest index not referencing a parameter.
/// The 𝖻𝗈𝖽𝗒 is an instruction sequence that upon termination must produce a stack matching the
/// function type’s result type.
/// Functions are referenced through function indices,
/// starting with the smallest index not referencing a function import.
pub struct Function {
    signature: TypeIndex,
    locals: Vec<ValueType>,
    body: Expression,
}

/// A table is a vector of opaque values of a particular reference type.
/// The 𝗆𝗂𝗇 size in the limits of the table type specifies the initial size of that table,
/// while its 𝗆𝖺𝗑, if present, restricts the size to which it can grow later.
pub struct Table(TableType);

/// A memory is a vector of raw uninterpreted bytes.
/// The 𝗆𝗂𝗇 size in the limits of the memory type specifies the initial size of that memory,
/// while its 𝗆𝖺𝗑, if present, restricts the size to which it can grow later.
/// Both are in units of page size.
pub struct Memory(MemoryType);

/// The 𝗀𝗅𝗈𝖻𝖺𝗅𝗌 component of a module defines a vector of global variables (or globals for short):
pub struct Global {
    global_type: GlobalType,
    initializer: Expression,
}

/// The initial contents of a table is uninitialized.
/// Element segments can be used to initialize a subrange of a table from a static vector of elements.
pub struct Element {
    reference_type: ReferenceType,
    initializers: Vec<Expression>,
    mode: ElementMode,
}

pub enum ElementMode {
    Passive,
    Active {
        table: TableIndex,
        offset: Expression,
    },
    Declarative,
}

/// The initial contents of a memory are zero bytes.
/// Data segments can be used to initialize a range of memory from a static vector of bytes.
pub struct Data {
    initializer: Vec<u8>,
    mode: DataMode,
}

pub enum DataMode {
    Passive,
    Active {
        memory: MemoryIndex,
        offset: Expression,
    },
}

/// The 𝗌𝗍𝖺𝗋𝗍 component of a module declares the function index of a start function that
/// is automatically invoked when the module is instantiated,
/// after tables and memories have been initialized.
pub struct Start(FunctionIndex);

/// Each export is labeled by a unique name.
/// Exportable definitions are functions, tables, memories, and globals,
/// which are referenced through a respective descriptor.
pub struct Export {
    name: Name,
    description: ExportDescription,
}

pub enum ExportDescription {
    Function(FunctionIndex),
    Table(TableIndex),
    Memory(MemoryIndex),
    Global(GlobalIndex),
}

/// Each import is labeled by a two-level name space, consisting of a 𝗆𝗈𝖽𝗎𝗅𝖾 name and a 𝗇𝖺𝗆𝖾 for an
/// entity within that module. Importable definitions are functions, tables, memories, and globals.
/// Each import is specified by a descriptor with a respective type that a definition provided
/// during instantiation is required to match.
/// Every import defines an index in the respective index space.
/// In each index space, the indices of imports go before the first index of any definition
/// contained in the module itself.
pub struct Import {
    module: Name,
    name: Name,
    description: ImportDescription,
}

pub enum ImportDescription {
    Function(TypeIndex),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}
