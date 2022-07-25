use std::convert::Infallible;
use std::io::ErrorKind::BrokenPipe;
use std::path::PathBuf;
use std::{env, fs};
use tortugac::walk::Walker;
use tortugac::{Program, SyntacticalError};

/// The name of the command-line interface executable.
pub const PROGRAM: &str = env!("CARGO_CRATE_NAME");
pub const INPUT_EXTENSION: &'static str = "ta";
pub const OUTPUT_EXTENSION: &'static str = "tb";

enum Error {
    IO(std::io::Error),
    Text(&'static str),
    Syntax(SyntacticalError),
    Infallible,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<&'static str> for Error {
    fn from(error: &'static str) -> Self {
        Error::Text(error)
    }
}

impl From<SyntacticalError> for Error {
    fn from(error: SyntacticalError) -> Self {
        Error::Syntax(error)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}

type CompilerResult = Result<(), Error>;

fn main() {
    let mut arguments = env::args();

    arguments.next(); // pop the command name

    match compile(arguments) {
        Ok(_) => (),
        Err(Error::IO(error)) if error.kind() == BrokenPipe => (),
        Err(Error::Infallible) => (),
        Err(Error::IO(error)) => eprintln!("{error}"),
        Err(Error::Text(error)) => eprintln!("{error}"),
        Err(Error::Syntax(error)) => eprintln!("{error}"),
    }
}

fn compile(mut arguments: env::Args) -> CompilerResult {
    match arguments.next() {
        Some(file_name) if arguments.len() == 0 => compile_file(file_name),
        _ => Err("Usage: {PROGRAM} [path]".into()),
    }
}

fn compile_file(file_path: String) -> CompilerResult {
    let mut path = PathBuf::from(file_path);

    match path.extension() {
        Some(extension) if extension == INPUT_EXTENSION => (),
        _ => return Err("Input file must have an extension of 'ta'.".into()),
    }

    let source = fs::read_to_string(path.as_path())?;
    let program: Program = source.as_str().parse()?;
    let emitter = tortugac::BinaryEmitter::default();

    if path.set_extension(OUTPUT_EXTENSION) {
        fs::write(path.as_path(), emitter.walk(program)?)?;
        Ok(())
    } else {
        Err("Invalid input file path.".into())
    }
}
