use crate::Program;
use std::str::FromStr;
use tortuga_executable::Executable;

mod error;
mod number;
mod uri;

pub use error::TranslationError;

impl<'a> TryFrom<&Program<'a>> for Executable {
    type Error = ();

    fn try_from(value: &Program<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}
