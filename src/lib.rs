pub mod about;
pub mod compiler;
mod errors;
mod fs;
pub mod syntax;

use crate::fs::CompilationSource;
pub use errors::TortugaError;
use futures::future::join_all;
use std::fmt::Debug;
use std::path::Path;

/// Cleans the given output directory.
#[tracing::instrument]
pub async fn clean<T: AsRef<Path> + Debug>(output: T) -> Result<(), TortugaError> {
    fs::clean(output).await
}

/// Compiles all of the Tortuga sources in the input directory.
/// The compilation artifacts are written to the output directory.
#[tracing::instrument]
pub async fn build<I, O>(input: I, output: O) -> Vec<Result<(), TortugaError>>
where
    I: AsRef<Path> + Debug,
    O: AsRef<Path> + Debug,
{
    let mut tasks = Vec::new();

    for source in fs::new_walker(input) {
        tasks.push(Box::pin(compile(source, &output)));
    }

    join_all(tasks).await
}

/// Compiles a single source.
#[tracing::instrument]
async fn compile<O: AsRef<Path> + Debug>(
    source: CompilationSource,
    output: O,
) -> Result<(), TortugaError> {
    tracing::info!(
        "Compiling {} to {}.",
        source.source().to_string_lossy(),
        source.target().to_string_lossy()
    );

    let mut source_file = source.source_file().await?;
    let mut target_file = source.target_file(&output).await?;

    compiler::compile(&mut source_file, &mut target_file).await?;

    Ok(())
}
