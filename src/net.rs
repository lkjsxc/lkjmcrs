use crate::config::Config;
use crate::session::ServerContext;
use tokio::net::TcpListener;

pub async fn serve(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(config.bind).await?;
    let context = ServerContext::new(config);
    tracing::info!(bind = %context.config.bind, "server listening");

    loop {
        let (stream, peer) = listener.accept().await?;
        let context = context.clone();
        tokio::spawn(async move {
            if let Err(error) = crate::session::handle_connection(stream, context).await {
                tracing::warn!(%peer, %error, "connection closed with error");
            }
        });
    }
}
