use crate::context::RequestContext;
use bytes::Bytes;
use std::io;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;

pub async fn serve(context: RequestContext, body: Bytes) -> io::Result<Bytes> {
    let mut child = Command::new(context.script()?)
        .kill_on_drop(true)
        .current_dir(context.working_directory())
        .args(context.arguments())
        .env_clear()
        .envs(context.variables())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut stdin = child.stdin.take();
    let mut stdout = child.stdout.take();

    let cancel = CancellationToken::new();
    let stdin_cancel = cancel.child_token();
    let _cancel_guard = cancel.drop_guard();

    let mut input = body.clone();

    tokio::spawn(async move {
        if let Some(mut stdin) = stdin.take() {
            select! {
                _ = stdin.write_all_buf(&mut input) => {}
                _ = stdin_cancel.cancelled() => {}
            }

            drop(stdin);
        }
    });

    let stdout_task = async {
        let mut output = Vec::with_capacity(1024 * 8);

        if let Some(stdout) = stdout.as_mut() {
            stdout.read_to_end(&mut output).await?;
        }

        Ok::<Vec<u8>, io::Error>(output)
    };

    match try_join!(
        tokio::time::timeout(Duration::from_secs(1), child.wait()),
        tokio::time::timeout(Duration::from_secs(1), stdout_task),
    ) {
        Ok((Ok(status), Ok(output))) if status.success() => Ok(Bytes::from(output)),
        Ok(_) => {
            child.kill().await?;

            Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Unable to wait for the child process to terminate.",
            ))
        }
        Err(_) => {
            child.kill().await?;

            Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Timed out waiting for the child process.",
            ))
        }
    }
}
