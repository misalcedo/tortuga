use crate::{Closure, Function, Value};
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

#[derive(Clone, Debug, PartialEq)]
pub struct CallFrame {
    return_to: usize,
    start_stack: usize,
    function: Function,
}

impl CallFrame {
    pub fn new(return_to: usize, start_stack: usize, function: Function) -> Self {
        CallFrame {
            return_to,
            start_stack,
            function,
        }
    }

    pub fn return_to(&self) -> usize {
        self.return_to
    }

    pub fn locals(&self) -> usize {
        self.function.locals()
    }

    pub fn values(&self) -> usize {
        self.function.values()
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
