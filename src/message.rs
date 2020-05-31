use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Envelope {
    to: String,
    message: Vec<u8>,
}

impl Envelope {
    pub fn new(to: Uuid, message: &[u8]) -> Envelope {
        Envelope {
            to: to.to_hyphenated().to_string(),
            message: message.iter().cloned().collect(),
        }
    }

    pub fn to(&self) -> Uuid {
        Uuid::parse_str(self.to.as_str()).unwrap()
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
