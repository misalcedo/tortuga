use crate::actor::error::Error;
use crate::actor::Actor;
use crate::wasm::Guest;
use std::convert::TryInto;

impl<T> Actor for T
where
    T: Guest,
{
    fn receive(&self, source: u128, message: &[u8]) -> Result<(), Error> {
        let length = message.len().try_into()?;
        let offset = self.allocate(length)?;

        self.write(offset, message)?;

        Ok(self.receive(source, offset, length)?)
    }
}
