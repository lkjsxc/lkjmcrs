use crate::config::Config;
use crate::session::{ConnectionLogLevel, ServerContext};
use tokio::net::TcpListener;

pub async fn serve(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let context = ServerContext::new(config)?;
    let listener = TcpListener::bind(context.config.bind).await?;
    tracing::info!(bind = %context.config.bind, "server listening");

    loop {
        let (stream, peer) = listener.accept().await?;
        let context = context.clone();
        tokio::spawn(async move {
            if let Err(error) = crate::session::handle_connection(stream, context).await {
                let phase = error.phase();
                match error.log_level() {
                    ConnectionLogLevel::Debug => {
                        tracing::debug!(%peer, phase = %phase, %error, "connection closed");
                    }
                    ConnectionLogLevel::Info => {
                        tracing::info!(%peer, phase = %phase, %error, "connection closed");
                    }
                    ConnectionLogLevel::Warn => {
                        tracing::warn!(%peer, phase = %phase, %error, "connection error");
                    }
                }
            }
        });
    }
}
