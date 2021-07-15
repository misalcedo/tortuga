mod compiler;
mod queue;
mod web_assembly;

use compiler::Compiler;
pub use queue::Envelope;
use std::fs::{create_dir_all, File};
use std::path::Path;

pub fn compile(input: &Path, output: &Path) {
    create_dir_all(output).unwrap();

    let compiler = Compiler::new();
    let filename = output.to_path_buf().join("example.wasm");
    let mut file = File::create(filename.as_path()).unwrap();
    let input: Vec<u8> = Vec::new();

    compiler.compile(&input.as_slice(), &mut file).unwrap();
}
