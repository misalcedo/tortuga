use clap::{Parser, Subcommand};
use std::path::Component::CurDir;
use std::path::PathBuf;
use tortuga::Server;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Options {
    /// Sets the verbosity of logging.
    #[arg(short = 'v', long = None, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Serve CGI scripts and static assets from an HTTP server.
    Serve(ServeOptions),
}

#[derive(Clone, Debug, Parser)]
struct ServeOptions {
    /// Enable an in-memory cache for compiled WebAssembly modules.
    #[arg(short, long)]
    wasm_cache: bool,

    /// Pre-load compiled WebAssembly modules into the in-memory cache.
    #[arg(short, long, requires = "wasm_cache")]
    preload_wasm: bool,

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

pub fn main() {
    let options = Options::parse();

    eprintln!("Starting server with options: {:?}", options);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match options.command {
        Some(Commands::Serve(serve_options)) => {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Unable to start an async runtime");

            let options = tortuga::Options {
                document_root: serve_options.document_root,
                cgi_bin: serve_options.cgi_bin,
                hostname: serve_options.hostname,
                port: serve_options.port,
            };

            runtime
                .block_on(async {
                    let server = Server::bind(options).await.unwrap();

                    println!("Server listening on port {}", server.address().unwrap());

                    server.serve().await
                })
                .expect("Unable to start the server");
        }
        _ => {}
    }
}
