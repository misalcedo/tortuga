mod extractors;
mod operators;
mod wrappers;

use crate::{Closure, Identifier, Number, Text};
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(Number),
    Text(Text),
    Closure(Closure),
    Identifier(Identifier),
    Reference(Rc<RefCell<Value>>),
}

impl Value {
    pub fn update(&mut self, value: Self) {
        match self {
            Value::Reference(reference) => *reference.borrow_mut() = value,
            _ => *self = value,
        }
    }

    pub fn capture(&mut self) -> Value {
        match self {
            Value::Reference(_) => self.clone(),
            _ => {
                *self = Value::Reference(Rc::new(RefCell::new(self.clone())));
                self.clone()
            }
        }
    }

    pub fn inner(self) -> Value {
        match self {
            Value::Reference(reference) => {
                let mut last = (*reference).borrow().deref().clone();

                while let Value::Reference(inner) = last {
                    last = (*inner).borrow().deref().clone();
                }

                last
            }
            _ => self,
        }
    }

    fn flatten(&self) -> Option<Ref<Self>> {
        let mut last = None;

        while let Value::Reference(inner) = self {
            last = Some((*inner).borrow());
        }

        last
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = self.flatten();
        let rhs = other.flatten();

        match (
            lhs.as_ref().map(Ref::deref).unwrap_or(self),
            rhs.as_ref().map(Ref::deref).unwrap_or(other),
        ) {
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Number(Number::default())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self.flatten();

        match value.as_ref().map(Ref::deref).unwrap_or(self) {
            Value::Number(n) => write!(f, "{}", n),
            Value::Closure(c) => write!(f, "{}", c),
            Value::Identifier(i) => write!(f, "{}", i),
            Value::Text(t) => write!(f, "{}", t),
            _ => unreachable!("A reference value should always resolve to a non-reference value."),
        }
    }
}
