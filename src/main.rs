use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;

mod about;
mod cgi;
mod server;

#[repr(transparent)]
#[derive(Clone, Debug)]
struct Interface(SocketAddr);

impl FromStr for Interface {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut addresses = s.to_socket_addrs()?;
        let address = addresses
            .next()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))?;

        Ok(Self(address))
    }
}

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

        /// The TCP host and port for the server to listen on
        #[arg(
            short,
            long,
            default_value = "localhost:3000",
            value_name = "INTERFACE"
        )]
        interface: Interface,
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
        Some(Commands::Serve { script, interface }) => {
            let server = server::Server::new(interface.0).unwrap();

            server.serve(script).expect("Unable to start the server");
        }
        Some(Commands::Test { script }) => {
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

            let child = cgi::spawn(&script, args, env).expect("Failed to read stdout");
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
