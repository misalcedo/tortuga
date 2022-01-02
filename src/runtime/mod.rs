//! Numbers, tuples, and other runtime data types necessary for compiling Tortuga programs.

mod environment;
mod epsilon;
mod error;
mod interpret;
mod number;
mod tolerance;
mod value;

pub use environment::{Environment, FunctionReference};
pub use epsilon::Epsilon;
pub use error::RuntimeError;
pub use interpret::Interpreter;
pub use number::Number;
pub use tolerance::Tolerance;
pub use value::Value;
