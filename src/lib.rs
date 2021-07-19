pub mod about;
mod compiler;
mod errors;
mod fs;
pub mod syntax;

use crate::fs::CompilationSource;
pub use errors::TortugaError;
use futures::future::join_all;
use std::path::Path;

/// Cleans the given output directory.
pub async fn clean<T: AsRef<Path>>(output: T) -> Result<(), TortugaError> {
    fs::clean(output).await
}

/// Compiles all of the Tortuga sources in the input directory.
/// The compilation artifacts are written to the output directory.
pub async fn build<I, O>(input: I, output: O) -> Vec<Result<(), TortugaError>>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    let mut tasks = Vec::new();

    for source in fs::new_walker(input) {
        tasks.push(Box::pin(compile(source, &output)));
    }

    join_all(tasks).await
}

/// Compiles a single source.
async fn compile<O: AsRef<Path>>(source: CompilationSource, output: O) -> Result<(), TortugaError> {
    let source_file = source.source_file().await?;
    let mut target_file = source.target_file(output).await?;

    compiler::compile(source_file, &mut target_file).await?;

    Ok(())
}
