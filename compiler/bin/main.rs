// mod error;
//
// use error::Error;
// use std::io::ErrorKind::BrokenPipe;
// use std::path::PathBuf;
// use std::{env, fs};
// use tortuga_compiler::walk::Walker;
// use tortuga_compiler::Program;
// use tracing::subscriber::set_global_default;
// use tracing::Level;
//
// pub const INPUT_EXTENSION: &'static str = "ta";
// pub const OUTPUT_EXTENSION: &'static str = "tb";
//
// type CompilerResult = Result<(), Error>;

fn main() {
    // let mut arguments = env::args();
    //
    // arguments.next(); // pop the command name
    //
    // match compile(arguments) {
    //     Ok(_) => (),
    //     Err(Error::IO(error)) if error.kind() == BrokenPipe => (),
    //     Err(Error::Infallible) => (),
    //     Err(Error::IO(error)) => eprintln!("{error}"),
    //     Err(Error::Text(error)) => eprintln!("{error}"),
    //     Err(Error::Syntax(error)) => eprintln!("{error}"),
    //     Err(Error::Tracing(error)) => eprintln!("Unable to set global tracing subscriber: {error}"),
    // }
}
//
// fn compile(mut arguments: env::Args) -> CompilerResult {
//     let collector = tracing_subscriber::fmt()
//         .with_max_level(Level::INFO)
//         .finish();
//
//     set_global_default(collector)?;
//
//     match arguments.next() {
//         Some(file_name) if arguments.len() == 0 => compile_file(file_name),
//         _ => Err("Usage: tortugac [path]".into()),
//     }
// }
//
// fn compile_file(file_path: String) -> CompilerResult {
//     let mut path = PathBuf::from(file_path);
//
//     match path.extension() {
//         Some(extension) if extension == INPUT_EXTENSION => (),
//         _ => return Err("Input file must have an extension of 'ta'.".into()),
//     }
//
//     let source = fs::read_to_string(path.as_path())?;
//     let program: Program = source.as_str().parse()?;
//     let emitter = tortuga_compiler::BinaryEmitter::default();
//
//     if path.set_extension(OUTPUT_EXTENSION) {
//         fs::write(path.as_path(), emitter.walk(program)?)?;
//         Ok(())
//     } else {
//         Err("Invalid input file path.".into())
//     }
// }
