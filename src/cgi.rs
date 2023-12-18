use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn run(script: &PathBuf) {
    println!("Running script {}...", script.display());

    let mut child = Command::new(script)
        .arg("-test")
        .arg("echo hello")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start script {script}");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    std::thread::spawn(move || {
        stdin
            .write_all("Hello, world!".as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Exit code: {}", output.status);
    println!("STDOUT: {stdout}");
    println!("STDERR: {stderr}");
}
