mod data;
mod element;
mod export;
mod expression;
mod function;
mod global;
mod identifier;
mod import;
mod instruction;
mod limit;
mod memory;
mod module;
mod offset;
mod start;
mod table;
mod text;
mod types;

pub use data::{Data, DataIndex};
pub use element::{Element, ElementIndex};
pub use export::Export;
pub use expression::Expression;
pub use function::{
    Function, FunctionIndex, LocalIndex, Parameter, Result, Type, TypeIndex, TypeUse,
};
pub use global::{Global, GlobalIndex, GlobalType};
pub use identifier::Identifier;
pub use import::Import;
pub use instruction::Instruction;
pub use limit::Limit;
pub use memory::{Memory, MemoryIndex, MemoryType, MemoryUse};
pub use module::Module;
pub use offset::Offset;
pub use start::Start;
pub use table::{Table, TableIndex, TableType, TableUse};
pub use text::{Name, String};
pub use types::{FloatType, IntegerType, NumberType, ReferenceType, ValueType};
