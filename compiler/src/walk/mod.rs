//! Walk the Abstract Syntax Tree.

mod emitter;

use crate::Program;
pub use emitter::BinaryEmitter;
use std::convert::Infallible;

pub trait Walker<R = Self, E = Infallible> {
    fn walk(self, program: Program) -> Result<R, E>;
}
