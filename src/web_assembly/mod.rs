mod instruction;
mod module;
mod types;
mod values;

pub use instruction::{Expression, Instruction};
pub use module::*;
pub use types::*;
pub use values::Name;

// TODO: Update syntax to follow the binary format not the text one. We will generate binary output.
