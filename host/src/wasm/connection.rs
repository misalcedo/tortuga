use crate::wasm;
use std::num::NonZeroU64;

#[derive(Clone)]
pub struct Connection<Primary, Factory, Rest> {
    primary: Primary,
    factory: Factory,
    rest: Vec<Rest>,
}

impl<Primary, Factory, Rest> Connection<Primary, Factory, Rest> {
    pub fn primary_mut(&mut self) -> &mut Primary {
        &mut self.primary
    }

    pub fn into_primary(self) -> Primary {
        self.primary
    }

    pub fn get_mut(&mut self, stream: NonZeroU64) -> Option<&mut Rest> {
        self.rest.get_mut(stream.get() as usize - 1)
    }
}

impl<Primary, Factory, Rest> Connection<Primary, Factory, Rest>
where
    Primary: wasm::Stream,
    Factory: wasm::Factory<Stream = Rest>,
    Rest: wasm::Stream,
{
    pub fn new(primary: Primary, factory: Factory) -> Self {
        Connection {
            factory,
            primary,
            rest: Vec::new(),
        }
    }

    pub fn start_stream(&mut self) -> NonZeroU64 {
        self.rest.push(self.factory.create());

        NonZeroU64::new(self.rest.len() as u64).unwrap()
    }
}
