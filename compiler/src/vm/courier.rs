use crate::{Identifier, Value};

pub struct NoOpCourier;

pub trait Courier {
    fn deliver(&mut self, to: Identifier, message: Value);
}

impl Courier for NoOpCourier {
    fn deliver(&mut self, _: Identifier, _: Value) {}
}

impl<F> Courier for F
where
    F: FnMut(Identifier, Value),
{
    fn deliver(&mut self, to: Identifier, message: Value) {
        println!("Deliver {:?} to {:?}", message, to);
    }
}
