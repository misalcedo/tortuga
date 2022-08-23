use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use tortuga::{CompilationError, Executable, RuntimeError, Value};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum PlaygroundError {
    Compilation(Vec<CompilationError>),
    Runtime(RuntimeError),
}

impl From<Vec<CompilationError>> for PlaygroundError {
    fn from(error: Vec<CompilationError>) -> Self {
        PlaygroundError::Compilation(error)
    }
}

impl From<RuntimeError> for PlaygroundError {
    fn from(error: RuntimeError) -> Self {
        PlaygroundError::Runtime(error)
    }
}

impl std::error::Error for PlaygroundError {}

impl Display for PlaygroundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlaygroundError::Runtime(error) => writeln!(f, "{}", error)?,
            PlaygroundError::Compilation(errors) => {
                for error in errors {
                    writeln!(f, "{}", error)?;
                }
            }
        }

        Ok(())
    }
}

#[wasm_bindgen]
pub fn run(input: &str) -> Result<String, String> {
    set_panic_hook();
    compile_and_run(input).map_err(|e| e.to_string())
}

pub fn compile_and_run(input: &str) -> Result<String, PlaygroundError> {
    let executable = Executable::from_str(input)?;
    let result = Value::try_from(executable)?;

    Ok(result.to_string())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
