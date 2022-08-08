mod error;
mod function;
mod number;
mod operation;
mod uri;

pub use error::{ParseNumberError, ParseUriError};
pub use function::Function;
pub use number::Number;
pub use operation::Operation;
pub use uri::Uri;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Executable {
    operations: Vec<Operation>,
    functions: Vec<Function>,
    numbers: Vec<Number>,
    uris: Vec<Uri>,
}
