use crate::Reference;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Envelope<'a> {
    pub to: String,
    pub message: &'a [u8],
}

impl<'a> Envelope<'a> {
    pub fn new(to: Reference, message: &[u8]) -> Envelope {
        Envelope {
            to: format!("{}", to),
            message,
        }
    }
}

impl<'a> Display for Envelope<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} >> {}",
            self.to.as_str(),
            String::from_utf8_lossy(self.message)
        )
    }
}
