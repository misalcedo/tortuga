use crate::{Function, Value};
use std::ops::{Index, IndexMut};

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
        self.function.locals() + 1
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

impl IndexMut<&CallFrame> for Vec<Value> {
    fn index_mut(&mut self, index: &CallFrame) -> &mut Self::Output {
        &mut self[index.start_stack..]
    }
}
