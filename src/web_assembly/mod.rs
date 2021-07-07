mod data;
mod element;
mod export;
mod expression;
mod function;
mod global;
mod identifier;
mod import;
mod instruction;
mod memory;
mod module;
mod offset;
mod start;
mod table;
mod types;
mod values;

pub use data::{Data, DataIndex};
pub use element::{Element, ElementIndex};
pub use export::Export;
pub use expression::Expression;
pub use function::{Function, FunctionIndex, LocalIndex, Type, TypeIndex, TypeUse};
pub use global::{Global, GlobalIndex};
pub use identifier::Identifier;
pub use import::Import;
pub use instruction::Instruction;
pub use memory::{Memory, MemoryIndex, MemoryUse};
pub use module::Module;
pub use offset::Offset;
pub use start::Start;
pub use table::{Table, TableIndex, TableUse};
pub use types::{
    FloatType, FunctionType, GlobalType, IntegerType, Limit, MemoryType, NumberType, ReferenceType,
    ResultType, TableType, ValueType,
};
pub use values::Name;

// TODO: Update syntax to follow the binary format not the text one. We will generate binary output.
