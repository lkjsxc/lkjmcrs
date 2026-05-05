use crate::config::Config;

pub async fn serve(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(bind = %config.bind, "server scaffold listening not implemented yet");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
