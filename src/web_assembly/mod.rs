mod data;
mod element;
mod export;
mod expression;
mod function;
mod global;
mod import;
mod instruction;
mod memory;
mod module;
mod offset;
mod start;
mod table;
mod types;
mod values;

pub use data::Data;
pub use export::Export;
pub use expression::Expression;
pub use import::Import;
pub use instruction::Instruction;
pub use module::*;
pub use offset::Offset;
pub use start::Start;
pub use types::*;
pub use values::Name;

// TODO: Update syntax to follow the binary format not the text one. We will generate binary output.
