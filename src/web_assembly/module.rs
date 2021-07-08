use crate::web_assembly::types::*;
use crate::web_assembly::{Expression, Name};

#[derive(Clone, Debug, PartialEq)]
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

    pub fn types(&self) -> &[FunctionType] {
        &self.types
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn memories(&self) -> &[Memory] {
        &self.memories
    }

    pub fn globals(&self) -> &[Global] {
        &self.globals
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    pub fn data(&self) -> &[Data] {
        &self.data
    }

    pub fn start(&self) -> Option<&Start> {
        self.start.as_ref()
    }

    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    pub fn exports(&self) -> &[Export] {
        &self.exports
    }
}

pub type TypeIndex = usize;
pub type FunctionIndex = usize;
pub type TableIndex = usize;
pub type MemoryIndex = usize;
pub type GlobalIndex = usize;
pub type ElementIndex = usize;
pub type DataIndex = usize;
pub type LocalIndex = usize;
pub type LabelIndex = usize;

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
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    kind: TypeIndex,
    locals: Vec<ValueType>,
    body: Expression,
}

impl Function {
    pub fn new(kind: TypeIndex, locals: Vec<ValueType>, body: Expression) -> Self {
        Function { kind, locals, body }
    }

    pub fn type_index(&self) -> TypeIndex {
        self.kind
    }
}

/// A table is a vector of opaque values of a particular reference type.
/// The 𝗆𝗂𝗇 size in the limits of the table type specifies the initial size of that table,
/// while its 𝗆𝖺𝗑, if present, restricts the size to which it can grow later.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Table {
    kind: TableType,
}

impl Table {
    pub fn new(kind: TableType) -> Self {
        Table { kind }
    }

    pub fn kind(&self) -> &TableType {
        &self.kind
    }
}

/// A memory is a vector of raw uninterpreted bytes.
/// The 𝗆𝗂𝗇 size in the limits of the memory type specifies the initial size of that memory,
/// while its 𝗆𝖺𝗑, if present, restricts the size to which it can grow later.
/// Both are in units of page size.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Memory {
    kind: MemoryType,
}

impl Memory {
    pub fn new(kind: MemoryType) -> Self {
        Memory { kind }
    }

    pub fn memory_type(&self) -> &MemoryType {
        &self.kind
    }
}

/// The 𝗀𝗅𝗈𝖻𝖺𝗅𝗌 component of a module defines a vector of global variables (or globals for short):
#[derive(Clone, Debug, PartialEq)]
pub struct Global {
    kind: GlobalType,
    initializer: Expression,
}

impl Global {
    pub fn new(kind: GlobalType, initializer: Expression) -> Self {
        Global { kind, initializer }
    }

    pub fn kind(&self) -> &GlobalType {
        &self.kind
    }

    pub fn initializer(&self) -> &Expression {
        &self.initializer
    }
}

/// The initial contents of a table is uninitialized.
/// Element segments can be used to initialize a subrange of a table from a static vector of elements.
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    kind: ElementKind,
    mode: ElementMode,
    initializers: ElementInitializer,
}

impl Element {
    pub fn new(kind: ElementKind, mode: ElementMode, initializers: ElementInitializer) -> Self {
        Element {
            kind,
            mode,
            initializers,
        }
    }

    pub fn kind(&self) -> &ElementKind {
        &self.kind
    }

    pub fn initializers(&self) -> &ElementInitializer {
        &self.initializers
    }

    pub fn mode(&self) -> &ElementMode {
        &self.mode
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementInitializer {
    Expression(Vec<Expression>),
    FunctionIndex(Vec<FunctionIndex>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ElementKind {
    FunctionReference,
    ReferenceType(ReferenceType),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ElementMode {
    Passive,
    Active(TableIndex, Expression),
    Declarative,
}

/// The initial contents of a memory are zero bytes.
/// Data segments can be used to initialize a range of memory from a static vector of bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    mode: DataMode,
    initializer: Vec<u8>,
}

impl Data {
    pub fn new(mode: DataMode, initializer: Vec<u8>) -> Self {
        Data { mode, initializer }
    }

    pub fn mode(&self) -> &DataMode {
        &self.mode
    }

    pub fn initializer(&self) -> &[u8] {
        &self.initializer
    }

    pub fn len(&self) -> usize {
        self.initializer.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataMode {
    Passive,
    Active(MemoryIndex, Expression),
}

/// The 𝗌𝗍𝖺𝗋𝗍 component of a module declares the function index of a start function that
/// is automatically invoked when the module is instantiated,
/// after tables and memories have been initialized.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Start {
    function: FunctionIndex,
}

impl Start {
    pub fn new(function: FunctionIndex) -> Self {
        Start { function }
    }

    pub fn function(&self) -> FunctionIndex {
        self.function
    }
}

/// Each export is labeled by a unique name.
/// Exportable definitions are functions, tables, memories, and globals,
/// which are referenced through a respective descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Export {
    name: Name,
    description: ExportDescription,
}

impl Export {
    pub fn new(name: Name, description: ExportDescription) -> Self {
        Export { name, description }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn description(&self) -> &ExportDescription {
        &self.description
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    module: Name,
    name: Name,
    description: ImportDescription,
}

impl Import {
    pub fn new(module: Name, name: Name, description: ImportDescription) -> Self {
        Import {
            module,
            name,
            description,
        }
    }

    pub fn module(&self) -> &Name {
        &self.module
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn description(&self) -> &ImportDescription {
        &self.description
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImportDescription {
    Function(TypeIndex),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_module() {
        let module = Module::new();

        assert!(module.types().is_empty());
        assert!(module.functions().is_empty());
        assert!(module.tables().is_empty());
        assert!(module.memories().is_empty());
        assert!(module.globals().is_empty());
        assert!(module.imports().is_empty());
        assert!(module.exports().is_empty());
        assert!(module.data().is_empty());
        assert!(module.elements().is_empty());
        assert!(module.start().is_none());
    }

    #[test]
    fn new_module() {}
}
