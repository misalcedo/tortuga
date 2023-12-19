use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

pub fn spawn<Arguments, Argument, Environment, Key, Value>(
    script: &PathBuf,
    arguments: Arguments,
    environment: Environment,
) -> Result<Child>
where
    Arguments: IntoIterator<Item = Argument>,
    Argument: AsRef<OsStr>,
    Environment: IntoIterator<Item = (Key, Value)>,
    Key: AsRef<OsStr>,
    Value: AsRef<OsStr>,
{
    Command::new(script)
        .args(arguments)
        .env_clear()
        .envs(environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}
