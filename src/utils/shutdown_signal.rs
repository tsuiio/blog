use once_cell::sync::Lazy;
use tokio::signal;
use tokio::sync::broadcast;
use tracing::warn;

pub static SHUTDOWN: Lazy<Shutdown> = Lazy::new(|| Shutdown::new());

pub struct Shutdown {
    sender: broadcast::Sender<()>,
}

impl Shutdown {
    pub fn new() -> Self {
        let (sender, _receiver) = broadcast::channel(1);
        let sender_clone = sender.clone();

        tokio::spawn(async move {
            Self::shutdown_signal().await;

            if let Err(_) = sender_clone.send(()) {
                warn!("Failed to broadcast shutdown signal");
            }
        });

        Shutdown { sender }
    }

    pub async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                warn!("Shutting down...");
            },
            _ = terminate => {
                warn!("Shutting down...");
            },
        }
    }

    pub async fn wait_for_shutdown(&self) {
        let mut receiver = self.sender.subscribe();
        let _ = receiver.recv().await;
    }
}
