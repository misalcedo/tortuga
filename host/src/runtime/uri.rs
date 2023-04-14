use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uri(String);

impl FromStr for Uri {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uri::from(s))
    }
}

impl From<String> for Uri {
    fn from(value: String) -> Self {
        Uri(value)
    }
}

impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Uri(value.to_string())
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
