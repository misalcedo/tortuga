use crate::machine::{Identifier, Value};

pub trait Courier {
    fn deliver(&self, to: Identifier, message: Value);
}

impl Courier for () {
    fn deliver(&self, to: Identifier, message: Value) {
        println!("Deliver {:?} to {:?}", message, to);
    }
}
