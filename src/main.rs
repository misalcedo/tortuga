use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cgi;
mod server;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Options {
    #[arg(short = 'v', long = None, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Serves a CGI script.
    Serve {
        /// Sets a CGI script file
        #[arg(short, long, value_name = "SCRIPT")]
        script: PathBuf,
    },
    /// Tests a CGI script.
    Test {
        /// Sets a CGI script file
        #[arg(short, long, value_name = "SCRIPT")]
        script: PathBuf,
    },
}

pub fn main() {
    let options = Options::parse();

    println!("Verbosity set to {}", options.verbosity);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &options.command {
        Some(Commands::Serve { script }) => {
            // Configure a runtime for the server that runs everything on the current thread
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("build runtime");

            // Combine it with a `LocalSet,  which means it can spawn !Send futures...
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, server::serve(script)).unwrap();
        }
        Some(Commands::Test { script }) => {
            cgi::run(script);
        }
        _ => {}
    }
}
