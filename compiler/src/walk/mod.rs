//! Walk the Abstract Syntax Tree.

use crate::Program;

pub trait Walker<T> {
    fn walk(program: Program) -> T;
}
