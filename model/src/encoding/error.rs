use std::fmt::{Debug, Display, Formatter};

pub type EncodingResult<Value, Error> = Result<Value, EncodingError<Error>>;

pub enum EncodingError<Error> {
    Encoding(Error),
    IO(std::io::Error),
}

impl<Error> From<std::io::Error> for EncodingError<Error> {
    fn from(value: std::io::Error) -> Self {
        EncodingError::IO(value)
    }
}

impl<Error> Debug for EncodingError<Error>
where
    Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::Encoding(e) => Debug::fmt(e, f),
            EncodingError::IO(e) => Debug::fmt(e, f),
        }
    }
}

impl<Error> Display for EncodingError<Error>
where
    Error: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::Encoding(e) => Display::fmt(e, f),
            EncodingError::IO(e) => Display::fmt(e, f),
        }
    }
}

impl<Error> std::error::Error for EncodingError<Error> where Error: std::error::Error {}
