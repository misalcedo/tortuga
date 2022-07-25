//! Walk the Abstract Syntax Tree.

mod emitter;

use crate::Program;
pub use emitter::BinaryEmitter;

pub trait Walker<T> {
    fn walk(&mut self, program: Program) -> T;
}
