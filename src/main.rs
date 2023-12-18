use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;

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

    eprintln!("Verbosity set to {}", options.verbosity);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match options.command {
        Some(Commands::Serve { script }) => {
            // Configure a runtime for the server that runs everything on the current thread
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("build runtime");

            let server = rt.spawn(server::serve(script));

            rt.block_on(handle_signals(server));
        }
        Some(Commands::Test { script }) => {
            let args = vec!["-test", "echo hello"];
            let env = HashMap::from([
                ("HTTP_HOST", "localhost"),
                ("HTTP_USER_AGENT", "curl/7.68.0"),
                ("HTTP_ACCEPT", "*/*"),
                ("CONTENT_LENGTH", "11"),
                ("CONTENT_TYPE", "application/x-www-form-urlencoded"),
                ("PATH", env!("PATH")),
                ("SERVER_SIGNATURE", "<address>Apache/2.4.41 (Ubuntu) Server at localhost Port 80</address>\n"),
                ("SERVER_SOFTWARE", "Apache/2.4.41 (Ubuntu)"),
                ("SERVER_NAME", "localhost"),
                ("SERVER_ADDR", "::1"),
                ("SERVER_PORT", "80"),
                ("REMOTE_ADDR", "::1"),
                ("DOCUMENT_ROOT", "/var/www/html"),
                ("REQUEST_SCHEME", "http"),
                ("CONTEXT_PREFIX", "/cgi-bin/"),
                ("CONTEXT_DOCUMENT_ROOT", "/usr/lib/cgi-bin/"),
                ("SERVER_ADMIN", "webmaster@localhost"),
                ("SCRIPT_FILENAME", "/usr/lib/cgi-bin/debug.cgi"),
                ("REMOTE_PORT", "55914"),
                ("GATEWAY_INTERFACE", "CGI/1.1"),
                ("SERVER_PROTOCOL", "HTTP/1.1"),
                ("REQUEST_METHOD", "POST"),
                ("QUERY_STRING", "foo+bar+--me%202"),
                ("REQUEST_URI", "/cgi-bin/debug.cgi/extra/path?foo+bar+--me%202"),
                ("SCRIPT_NAME", "/cgi-bin/debug.cgi"),
                // Only if there is a path after the script portion of the path.
                // Translates the extra path based on the rules of the server to a local path.
                ("PATH_INFO", "/extra/path"),
                ("PATH_TRANSLATED", "/var/www/html/extra/path")
            ]);

            let output = cgi::run(&script, args, env).expect("Failed to read stdout");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
        
            println!("Exit code: {}", output.status);
            println!("STDOUT:\n{stdout}");
            println!("STDERR:\n{stderr}");
        }
        _ => {}
    }
}

async fn handle_signals(server: tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>) {
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
           eprintln!("Shutting down the server...");
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    server.abort();

    if let Err(e) = server.await {
        eprintln!("{e}");
    }

    exit(0);
}