use std::error::Error;
use std::fs::read;
use std::path::PathBuf;

use structopt::StructOpt;
use tokio::net::UdpSocket;
use uuid::Uuid;

use tortuga::Envelope;

#[derive(Debug, StructOpt)]
struct Act {
    #[structopt(short, long, parse(from_os_str))]
    pub intent: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tortuga",
    about = "An actor-based system built on top of WASM technologies."
)]
enum Tortuga {
    Act(Act),
}

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    match Tortuga::from_args() {
        Tortuga::Act(options) => act(options).await,
    }
}

async fn act(options: Act) -> Result<(), Box<dyn Error>> {}
