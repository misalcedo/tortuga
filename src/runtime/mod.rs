//! Numbers, tuples, and other runtime data types necessary for compiling Tortuga programs.

mod epsilon;
mod error;
mod number;
mod tolerance;
mod value;

pub use epsilon::EpsilonOperator;
pub use error::RuntimeError;
pub use number::Number;
pub use tolerance::Tolerance;
pub use value::Value;
