use crate::{Closure, Value};
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

#[derive(Clone, Debug, PartialEq)]
pub struct CallFrame {
    return_to: usize,
    start_stack: usize,
    start_frame: usize,
}

impl CallFrame {
    pub fn new(return_to: usize, start_stack: usize, start_frame: usize) -> Self {
        CallFrame {
            return_to,
            start_stack,
            start_frame,
        }
    }

    pub fn return_to(&self) -> usize {
        self.return_to
    }

    pub fn start(&self) -> usize {
        self.start_frame
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
        &self[index.start_stack..]
    }
}
