use crate::walk::Walker;
use crate::Program;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinaryEmitter {
    contents: Vec<u8>,
}

impl Walker<&[u8]> for BinaryEmitter {
    fn walk(&mut self, _program: Program) -> &[u8] {
        self.contents.as_slice()
    }
}
