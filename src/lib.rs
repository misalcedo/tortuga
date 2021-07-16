use std::fs::{create_dir_all, remove_dir_all, File};
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

use compiler::Compiler;
pub use errors::TortugaError;

mod compiler;
mod errors;
mod queue;
mod syntax;

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

pub fn clean(output: &Path) -> Result<(), TortugaError> {
    Ok(remove_dir_all(output)?)
}

pub fn build(input: &Path, output: &Path) -> Result<(), TortugaError> {
    let walker = WalkDir::new(input)
        .follow_links(false)
        .into_iter()
        .filter_entry(is_not_hidden)
        .filter_map(|e| e.ok());
    let compiler = Compiler::new();

    create_dir_all(output)?;

    for entry in walker {
        let filename = entry
            .file_name()
            .to_str()
            .ok_or_else(|| TortugaError::InvalidFileName(entry.file_name().to_os_string()))?;

        if filename.ends_with(".ta") {
            let output_path = entry.path().strip_prefix(input)?;
            let mut filename = output
                .to_path_buf()
                .join(output_path)
                .with_extension("wasm");

            if let Some(parent) = filename.as_path().parent() {
                create_dir_all(parent)?;
            }

            println!("Input File: {}", entry.path().to_string_lossy());
            println!("Output File: {}", filename.as_path().to_string_lossy());

            let input_file = File::open(entry.path())?;
            let mut output_file = File::create(filename.as_path())?;

            compiler.compile(&input_file, &mut output_file)?;
        }
    }

    Ok(())
}
