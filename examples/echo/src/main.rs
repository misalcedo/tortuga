use std::io::{self, stdin, stdout, Write};

fn main() -> io::Result<()> {
    let mut output = stdout();

    let content_type = std::env::var("CONTENT_TYPE").unwrap_or_else(|_| "text/html".to_string());
    let content_length =
        std::env::var("CONTENT_LENGTH").unwrap_or_else(|_| "text/html".to_string());

    write!(output, "Content-Type: {}\n", content_type)?;
    write!(output, "Content-Length: {}\n", content_length)?;
    write!(output, "\n")?;

    io::copy(&mut stdin(), &mut output).map(|_| ())
}
