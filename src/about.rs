//! Information about the tortuga program.

/// The name of the command-line interface executable.
pub const PROGRAM: &str = env!("CARGO_CRATE_NAME");

/// The full (major, minor, and path) version of Tortuga.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
