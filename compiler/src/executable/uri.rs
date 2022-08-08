use crate::executable::ParseUriError;
use crate::grammar;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Uri(String);

impl FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uri(s.to_string()))
    }
}

impl<'a> TryFrom<grammar::Uri<'a>> for Uri {
    type Error = ParseUriError;

    fn try_from(uri: grammar::Uri<'a>) -> Result<Self, Self::Error> {
        Ok(Uri(uri.as_str().to_string()))
    }
}
