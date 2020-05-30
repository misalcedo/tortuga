use crate::Reference;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Envelope {
    to: String,
    message: Vec<u8>,
}

impl Envelope {
    pub fn new(to: Reference, message: &[u8]) -> Envelope {
        Envelope {
            to: format!("{}", to),
            message: message.iter().cloned().collect()
        }
    }

    pub fn to(&self) -> Reference {
        Reference::from(self.to.as_str())
    }

    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

impl Display for Envelope {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} >> {}",
            self.to.as_str(),
            String::from_utf8_lossy(self.message())
        )
    }
}
