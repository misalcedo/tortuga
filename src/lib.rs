pub mod about;
mod compiler;
mod errors;
mod fs;
mod syntax;

use compiler::Compiler;
pub use errors::TortugaError;
pub use fs::clean;
use std::path::Path;

/// Compiles all of the Tortuga sources in the input directory.
/// The compilation artifacts are written to the output directory.
pub fn build<I, O>(input: I, output: O) -> Result<(), TortugaError>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let compiler = Compiler::new();

    for source in fs::new_walker(input) {
        let source_file = source.source_file()?;
        let mut target_file = source.target_file(&output)?;

        compiler.compile(source_file, &mut target_file)?;
    }

    Ok(())
}
