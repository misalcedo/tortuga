use crate::actor::error::Error;
use crate::actor::Actor;
use crate::wasm::Guest;

impl<T> Actor for T
where
    T: Guest,
{
    fn receive(&self, message: &[u8]) -> Result<(), Error> {
        let address = self.copy(message)?;

        Ok(self.receive(address)?)
    }
}
