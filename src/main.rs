use clap::{Parser, Subcommand};
use http::uri::Uri;
use std::path::Component::CurDir;
use std::path::PathBuf;

mod about;
mod context;
mod script;
mod server;
mod uri;
mod variable;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Options {
    /// Sets the verbosity of logging.
    #[arg(short = 'v', long = None, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Parser)]
struct ServeOptions {
    /// The path to a cache directory for WASM CGI script compilation.
    /// Relative paths are resolved from the current working directory.
    #[arg(short, long, value_name = "WASM_CACHE")]
    wasm_cache: Option<PathBuf>,

    /// The document root path to load CGI scripts and other assets from.
    #[arg(value_name = "DOCUMENT_ROOT")]
    document_root: PathBuf,

    /// The path to CGI scripts; may be relative or absolute.
    /// Relative paths are resolved from the document root.
    #[arg(short, long, default_value=CurDir.as_os_str(), value_name = "CGI_BIN")]
    cgi_bin: PathBuf,

    /// The hostname of the local TCP interface for the server to listen on.
    #[arg(
        short = 'H',
        long,
        default_value = "localhost",
        value_name = "HOSTNAME"
    )]
    hostname: String,

    /// The TCP port for the server to listen on.
    #[arg(short = 'P', long, default_value_t = 0, value_name = "PORT")]
    port: u16,
}

#[derive(Parser)]
struct InvokeOptions {
    /// The path to the CGI script to invoke.
    #[arg(short, long, value_name = "SCRIPT_PATH", required = true)]
    script: PathBuf,

    /// The URI to simulate the script invocation for.
    #[arg(short, long, value_name = "URI", required = true)]
    uri: Uri,
}

#[derive(Subcommand)]
enum Commands {
    /// Serve CGI scripts and static assets from an HTTP server.
    Serve(ServeOptions),
    /// Invoke a single CGI script.
    Invoke(InvokeOptions),
}

pub fn main() {
    let options = Options::parse();

    eprintln!("Verbosity set to {}", options.verbosity);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match options.command {
        Some(Commands::Serve(options)) => {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Unable to start an async runtime");

            runtime
                .block_on(async {
                    let server = server::Server::bind(options).await.unwrap();

                    println!("Server listening on port {}", server.address().unwrap());

                    server.serve().await
                })
                .expect("Unable to start the server");
        }
        Some(Commands::Invoke(_options)) => {
            todo!()
        }
        _ => {}
    }
}
