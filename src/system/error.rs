use crate::system::ActorReference;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to send a message from '{0}' to '{1}'.")]
    FailedToSend(ActorReference, ActorReference),
}
