use tokio_util::sync::CancellationToken;

#[repr(transparent)]
#[derive(Clone)]
pub struct ShutdownSignal(CancellationToken);

impl ShutdownSignal {
    pub fn new() -> Self {
        Self(CancellationToken::new())
    }

    pub fn shutdown(self) {
        self.0.cancel()
    }

    pub async fn shutdown_requested(&self) {
        self.0.cancelled().await
    }
}
