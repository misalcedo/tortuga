use clap::{Parser, Subcommand};
use http::uri::Uri;
use std::collections::HashMap;
use std::path::Component::CurDir;
use std::path::PathBuf;

mod about;
mod cgi;
mod context;
mod server;
mod service;
mod variable;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Options {
    #[arg(short = 'v', long = None, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Parser)]
struct ServeOptions {
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
        Some(Commands::Invoke(options)) => {
            let args = vec!["-test", "echo hello"];
            let env = HashMap::from([
                ("PATH", env!("PATH")),
                (
                    "SERVER_SIGNATURE",
                    "<address>Apache/2.4.41 (Ubuntu) Server at localhost Port 80</address>\n",
                ),
                ("SERVER_SOFTWARE", "Apache/2.4.41 (Ubuntu)"),
                ("SERVER_NAME", "localhost"),
                ("SERVER_ADDR", "::1"),
                ("SERVER_PORT", "80"),
                ("REMOTE_ADDR", "::1"),
                ("DOCUMENT_ROOT", "/var/www/html"),
                ("CONTEXT_PREFIX", "/cgi-bin/"),
                ("CONTEXT_DOCUMENT_ROOT", "/usr/lib/cgi-bin/"),
                ("SERVER_ADMIN", "webmaster@localhost"),
                ("SCRIPT_FILENAME", "/usr/lib/cgi-bin/debug.cgi"),
                ("REMOTE_PORT", "55914"),
                ("GATEWAY_INTERFACE", "CGI/1.1"),
                ("SERVER_PROTOCOL", "HTTP/1.1"),
                // HTTP
                ("HTTP_HOST", "localhost"),
                ("HTTP_USER_AGENT", "curl/7.68.0"),
                ("HTTP_ACCEPT", "*/*"),
                ("CONTENT_LENGTH", "11"),
                ("CONTENT_TYPE", "application/x-www-form-urlencoded"),
                ("REQUEST_SCHEME", "http"),
                ("REQUEST_METHOD", "POST"),
                ("SCRIPT_NAME", "/cgi-bin/debug.cgi"),
                ("QUERY_STRING", "foo+bar+--me%202"),
                (
                    "REQUEST_URI",
                    "/cgi-bin/debug.cgi/extra/path?foo+bar+--me%202",
                ),
                // Only if there is a path after the script portion of the path.
                // Translates the extra path based on the rules of the server to a local path.
                ("PATH_INFO", "/extra/path"),
                ("PATH_TRANSLATED", "/var/www/html/extra/path"),
            ]);

            let child = cgi::spawn(&options.script, args, env).expect("Failed to read stdout");
            let output = child
                .wait_with_output()
                .expect("Failed to wait for the child process.");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            println!("Exit code: {}", output.status);
            println!("STDOUT:\n{stdout}");
            println!("STDERR:\n{stderr}");
        }
        _ => {}
    }
}
