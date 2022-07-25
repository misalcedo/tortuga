use crate::walk::Walker;
use crate::Program;
use std::convert::Infallible;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinaryEmitter {
    contents: Vec<u8>,
}

impl Walker<Vec<u8>> for BinaryEmitter {
    fn walk(self, _program: Program) -> Result<Vec<u8>, Infallible> {
        Ok(self.contents)
    }
}
