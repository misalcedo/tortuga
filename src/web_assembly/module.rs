use crate::web_assembly::types::*;
use crate::web_assembly::{Expression, Name};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    function_types: Vec<FunctionType>,
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
            function_types: Vec::new(),
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

    pub fn function_types(&self) -> &[FunctionType] {
        &self.function_types
    }

    pub fn add_function_type(&mut self, function_type: FunctionType) {
        self.function_types.push(function_type);
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn memories(&self) -> &[Memory] {
        &self.memories
    }

    pub fn add_memory(&mut self, memory: Memory) {
        self.memories.push(memory);
    }

    pub fn globals(&self) -> &[Global] {
        &self.globals
    }

    pub fn add_global(&mut self, global: Global) {
        self.globals.push(global);
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn data(&self) -> &[Data] {
        &self.data
    }

    pub fn add_data(&mut self, data: Data) {
        self.data.push(data);
    }

    pub fn start(&self) -> Option<&Start> {
        self.start.as_ref()
    }

    pub fn set_start(&mut self, start: Option<Start>) {
        self.start = start;
    }

    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    pub fn exports(&self) -> &[Export] {
        &self.exports
    }

    pub fn add_export(&mut self, export: Export) {
        self.exports.push(export);
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
    locals: ResultType,
    body: Expression,
}

impl Function {
    pub fn new(kind: TypeIndex, locals: ResultType, body: Expression) -> Self {
        Function { kind, locals, body }
    }

    pub fn kind(&self) -> TypeIndex {
        self.kind
    }

    pub fn locals(&self) -> &ResultType {
        &self.locals
    }

    pub fn body(&self) -> &Expression {
        &self.body
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

    pub fn kind(&self) -> &MemoryType {
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
    use crate::web_assembly::Instruction;

    #[test]
    fn empty_module() {
        let module = Module::new();

        assert!(module.function_types().is_empty());
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
    fn module() {
        let mut module = Module::new();
        let function_type = FunctionType::new(
            ResultType::new(vec![ValueType::Number(NumberType::I64)]),
            ResultType::new(vec![ValueType::Number(NumberType::F64)]),
        );
        module.add_function_type(function_type.clone());

        let function = Function::new(
            0,
            ResultType::new(vec![ValueType::Number(NumberType::I32)]),
            Expression::new(vec![Instruction::Nop]),
        );
        module.add_function(function.clone());

        let element = Element::new(
            ElementKind::ReferenceType(ReferenceType::Function),
            ElementMode::Passive,
            ElementInitializer::FunctionIndex(vec![0]),
        );
        module.add_element(element.clone());

        let data = Data::new(DataMode::Passive, vec![42]);
        module.add_data(data.clone());

        let table = Table::new(TableType::new(Limit::new(0, None), ReferenceType::Function));
        module.add_table(table);

        let memory = Memory::new(MemoryType::new(Limit::new(0, None)));
        module.add_memory(memory);

        let import = Import::new(
            Name::new("test".to_string()),
            Name::new("foobar".to_string()),
            ImportDescription::Function(0),
        );
        module.add_import(import.clone());

        let export = Export::new(
            Name::new("foobar".to_string()),
            ExportDescription::Function(0),
        );
        module.add_export(export.clone());

        let start = Start::new(0);
        module.set_start(Some(start));

        let global = Global::new(
            GlobalType::new(false, ValueType::Number(NumberType::I64)),
            Expression::new(vec![Instruction::I64Constant(0)]),
        );
        module.add_global(global.clone());

        assert_eq!(module.function_types(), &[function_type]);
        assert_eq!(module.functions(), &[function]);
        assert_eq!(module.tables(), &[table]);
        assert_eq!(module.memories(), &[memory]);
        assert_eq!(module.globals(), &[global]);
        assert_eq!(module.imports(), &[import]);
        assert_eq!(module.exports(), &[export]);
        assert_eq!(module.data(), &[data]);
        assert_eq!(module.elements(), &[element]);
        assert_eq!(module.start(), Some(&start));
    }

    #[test]
    fn new_function() {
        let kind = 1;
        let locals = ResultType::new(vec![ValueType::Number(NumberType::I64)]);
        let body = Expression::new(Vec::new());
        let function = Function::new(kind, locals.clone(), body.clone());

        assert_eq!(function.kind(), 1);
        assert_eq!(function.locals(), &locals);
        assert_eq!(function.body(), &body);
    }

    #[test]
    fn new_elements() {
        let kind = ElementKind::ReferenceType(ReferenceType::Function);
        let mode = ElementMode::Active(0, Expression::new(Vec::new()));
        let initializers = ElementInitializer::FunctionIndex(vec![1]);
        let element = Element::new(kind, mode.clone(), initializers.clone());

        assert_eq!(element.mode(), &mode);
        assert_eq!(element.initializers(), &initializers);
        assert_eq!(element.kind(), &kind);
    }

    #[test]
    fn new_data() {
        let mode = DataMode::Active(0, Expression::new(Vec::new()));
        let initializer = vec![42];
        let data = Data::new(mode.clone(), initializer.clone());

        assert_eq!(data.mode(), &mode);
        assert_eq!(data.initializer(), &initializer);
        assert_eq!(data.len(), initializer.len());
    }

    #[test]
    fn new_table() {
        let kind = TableType::new(Limit::new(0, None), ReferenceType::Function);
        let table = Table::new(kind);

        assert_eq!(table.kind(), &kind);
    }

    #[test]
    fn new_memory() {
        let kind = MemoryType::new(Limit::new(0, None));
        let memory = Memory::new(kind);

        assert_eq!(memory.kind(), &kind);
    }

    #[test]
    fn new_import() {
        let module = Name::new("test".to_string());
        let name = Name::new("foobar".to_string());
        let kind = MemoryType::new(Limit::new(0, None));
        let description = ImportDescription::Memory(kind);
        let import = Import::new(module.clone(), name.clone(), description);

        assert_eq!(import.module(), &module);
        assert_eq!(import.name(), &name);
        assert_eq!(import.description(), &description);
    }

    #[test]
    fn new_export() {
        let name = Name::new("foobar".to_string());
        let description = ExportDescription::Function(42);
        let export = Export::new(name.clone(), description);

        assert_eq!(export.name(), &name);
        assert_eq!(export.description(), &description);
    }

    #[test]
    fn new_start() {
        let function = 42;
        let start = Start::new(function);

        assert_eq!(start.function(), function);
    }

    #[test]
    fn new_global() {
        let kind = GlobalType::new(true, ValueType::Number(NumberType::I64));
        let expression = Expression::new(Vec::new());
        let global = Global::new(kind, expression.clone());

        assert_eq!(global.kind(), &kind);
        assert_eq!(global.initializer(), &expression);
    }
}
