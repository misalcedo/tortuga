use std::io::ErrorKind::BrokenPipe;
use std::path::PathBuf;
use std::{env, fs};
use tortugac::walk::Walker;
use tortugac::Program;

/// The name of the command-line interface executable.
pub const PROGRAM: &str = env!("CARGO_CRATE_NAME");
pub const INPUT_EXTENSION: &'static str = "ta";
pub const OUTPUT_EXTENSION: &'static str = "tb";

fn main() {
    let mut arguments = env::args();

    arguments.next(); // pop the command name

    match compile(arguments) {
        Err(error @ std::io::Error) if error.kind() == BrokenPipe => (),
        Err(error) => eprintln!("{error}"),
        _ => (),
    }
}

fn compile(mut arguments: env::Args) -> Result<(), Box<dyn std::error::Error>> {
    match arguments.next() {
        Some(file_name) if arguments.len() == 0 => compile_file(file_name),
        None => read_evaluate_print_loop(),
        Some(_) => Err(Box::new("Usage: {PROGRAM} [path]")),
    }
}

fn compile_file(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(file_path);

    match path.extension() {
        Some(extension) if extension == INPUT_EXTENSION => (),
        Some(extension) => Err(Box::new("Invalid input file extension '{extension}'.")),
        None => Err(Box::new(
            "Input file must have an extension of {INPUT_EXTENSION}.",
        )),
    }

    let source = fs::read_to_string(path.as_path())?;
    let program: Program = source.as_str().parse()?;
    let mut emitter = tortugac::BinaryEmitter::default();

    if output.set_extension(OUTPUT_EXTENSION) {
        fs::write(path.as_path(), emitter.walk(program))?;
        Ok(())
    } else {
        Err(Box::new("Invalid input file path."))
    }
}
