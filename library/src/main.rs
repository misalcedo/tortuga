extern crate tortuga;

use tortuga::errors::WasmError;

fn main() -> Result<(), WasmError> {
    tortuga::run()
}
