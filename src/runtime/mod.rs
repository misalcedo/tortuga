//! Numbers, tuples, and other runtime data types necessary for compiling Tortuga programs.

mod environment;
mod epsilon;
mod error;
mod function;
mod interpret;
mod number;
mod tolerance;
mod value;

pub use environment::{Environment, FunctionReference};
pub use epsilon::EpsilonOperator;
pub use error::RuntimeError;
pub use function::Function;
pub use interpret::Interpreter;
pub use number::Number;
pub use tolerance::Tolerance;
pub use value::Value;
