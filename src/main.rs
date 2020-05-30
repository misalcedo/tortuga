use std::error::Error;
use std::fs::read;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::net::UdpSocket;
use tortuga::Envelope;

#[derive(Debug, StructOpt)]
struct Send {
    #[structopt(short, long)]
    pub reference: String,
    #[structopt(short, long, default_value = "Hello, World!")]
    pub message: String,
    #[structopt(short, long, default_value = "localhost:2867")]
    pub system: String,
    #[structopt(short, long, default_value = "localhost:0")]
    pub address: String,
}

#[derive(Debug, StructOpt)]
struct Act {
    #[structopt(
        short,
        long,
        default_value = "./resources/wasm/echo.wat",
        parse(from_os_str)
    )]
    pub intent: PathBuf,
    #[structopt(short, long, default_value = "localhost:2867")]
    pub address: String,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tortuga",
    about = "An actor-based system built on top of WASM technologies."
)]
enum Tortuga {
    Act(Act),
    Send(Send),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match Tortuga::from_args() {
        Tortuga::Act(options) => act(options).await,
        Tortuga::Send(options) => send(options).await,
    }
}

const MAX_DATAGRAM: usize = 65535;

async fn act(options: Act) -> Result<(), Box<dyn Error>> {
    let mut system = tortuga::System::new();
    let intent = read(options.intent)?;
    let actor = system.register(&intent)?;
    let mut socket = UdpSocket::bind(options.address).await?;

    println!("Created actor system with reference: {}", actor);

    let mut buffer = [0u8; MAX_DATAGRAM];

    while let Ok((read, from)) = socket.recv_from(&mut buffer).await {
        let payload = &buffer[..read];
        let envelope: Envelope = postcard::from_bytes(payload)?;

        println!("Received '{}' from {}.", envelope, from);

        let reference = envelope.to();

        system.send(envelope)?;
        system.run(reference)?;
    }

    Ok(())
}

async fn send(options: Send) -> Result<(), Box<dyn Error>> {
    let actor = tortuga::Reference::from(options.reference.as_str());
    let mut system = UdpSocket::bind(options.address).await?;
    let mut buffer = [0u8; MAX_DATAGRAM];

    system.connect(options.system.as_str()).await?;

    let envelope = Envelope::new(actor, options.message.as_bytes());
    let payload = postcard::to_slice(&envelope, &mut buffer)?;
    let sent = system.send(payload).await?;

    println!(
        "Sent {} bytes to system at udp://{}",
        sent,
        options.system.as_str()
    );

    Ok(())
}
