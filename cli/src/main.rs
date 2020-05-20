use std::error::Error;
use std::fs::read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tortuga",
    about = "An actor-based system built on top of WASM technologies."
)]
struct Options {
    #[structopt(
        short,
        long,
        default_value = "./resources/wasm/echo.wat",
        parse(from_os_str)
    )]
    actor: PathBuf,
    #[structopt(short, long, default_value = "Hello, World!")]
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();
    let echo_module = &read(options.actor)?;

    let mut system = tortuga::System::new();
    let actor = system.register(echo_module)?;

    system.send(actor, options.message.as_bytes())?;
    system.run(actor)?;

    Ok(())
}
