//! Information about the tortuga command.

/// The name of the command-line interface executable.
pub const PROGRAM: &str = env!("CARGO_CRATE_NAME");

/// The full (major, minor, and path) version of Tortuga.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Colon separated list of Tortuga's authors.
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// A short description of Tortuga.
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const UNUSED  : u8 =   0    ;