use std::ffi::OsStr;
use std::io::{Result, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

pub fn run<Arguments, Argument, Environment, Key, Value>(script: &PathBuf, arguments: Arguments, environment: Environment) -> Result<Output>
where
    Arguments: IntoIterator<Item = Argument>,
    Argument: AsRef<OsStr>,
    Environment: IntoIterator<Item = (Key, Value)>,
    Key: AsRef<OsStr>,
    Value: AsRef<OsStr>,
{
    let mut child = Command::new(script)
        .args(arguments)
        .env_clear()
        .envs(environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(format!("failed to start script {}", script.display()).as_str());

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    std::thread::spawn(move || {
        stdin
            .write_all("Hello, world!".as_bytes())
            .expect("Failed to write to stdin");
    });

    child.wait_with_output()
}
