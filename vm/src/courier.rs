use crate::{Identifier, Value};

pub trait Courier {
    fn deliver(&mut self, to: Identifier, message: Value);
}

impl Courier for () {
    fn deliver(&mut self, to: Identifier, message: Value) {
        println!("Deliver {:?} to {:?}", message, to);
    }
}

impl<F> Courier for F
where
    F: FnMut(Identifier, Value),
{
    fn deliver(&mut self, to: Identifier, message: Value) {
        println!("Deliver {:?} to {:?}", message, to);
    }
}
