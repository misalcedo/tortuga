use crate::{Function, Value};
use std::ops::{Index, IndexMut, Range, RangeFrom};
use std::rc::Rc;

/// Closure, Parameters, Locals, Captures
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CallFrame {
    code: Rc<Vec<u8>>,
    start_stack: usize,
    parameters: usize,
    captures: usize,
    locals: usize,
    defined_locals: usize,
    cursor: usize,
}

impl CallFrame {
    pub fn new(start_stack: usize, function: &Function) -> Self {
        CallFrame {
            start_stack,
            code: function.code(),
            parameters: function.arity(),
            captures: function.captures().len(),
            locals: function.locals(),
            defined_locals: 0,
            cursor: 0,
        }
    }

    pub fn define_local(&mut self) -> Result<usize, usize> {
        if self.defined_locals < self.locals {
            self.defined_locals += 1;

            Ok(self.parameters + self.defined_locals)
        } else {
            Err(self.locals)
        }
    }

    pub fn locals(&self) -> Range<usize> {
        self.start_stack..self.start_captures()
    }

    pub fn captures(&self) -> Range<usize> {
        self.start_captures()..self.end_frame()
    }

    pub fn temporaries(&self) -> RangeFrom<usize> {
        self.end_frame()..
    }

    pub fn all(&self) -> RangeFrom<usize> {
        self.start_stack..
    }

    pub fn jump(&mut self, offset: usize) {
        self.cursor += offset;
    }

    fn end_frame(&self) -> usize {
        self.start_captures() + self.captures
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        self.next()
    }

    pub fn read_u16(&mut self) -> Result<u16, usize> {
        let bytes = self.read::<2>()?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn read<const N: usize>(&mut self) -> Result<[u8; N], usize> {
        let mut bytes = [0u8; N];

        for (index, x) in bytes.iter_mut().enumerate() {
            match self.next() {
                None => return Err(index),
                Some(byte) => *x = byte,
            }
        }

        Ok(bytes)
    }

    fn start_captures(&self) -> usize {
        self.start_stack + 1 + self.parameters + self.defined_locals
    }
}

impl Iterator for CallFrame {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.code.get(self.cursor)?;

        self.cursor += 1;

        Some(*byte)
    }
}

impl Index<CallFrame> for Vec<Value> {
    type Output = [Value];

    fn index(&self, index: CallFrame) -> &Self::Output {
        &self[index.start_stack..]
    }
}

impl Index<&CallFrame> for Vec<Value> {
    type Output = [Value];

    fn index(&self, index: &CallFrame) -> &Self::Output {
        &self[index.start_stack..index.end_frame()]
    }
}

impl IndexMut<&CallFrame> for Vec<Value> {
    fn index_mut(&mut self, index: &CallFrame) -> &mut Self::Output {
        &mut self[index.start_stack..index.end_frame()]
    }
}
