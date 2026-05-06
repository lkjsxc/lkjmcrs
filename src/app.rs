use crate::config::Config;
use crate::quality;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lkjmcrs")]
#[command(about = "Rust Minecraft server skeleton")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve,
    Docs {
        #[command(subcommand)]
        command: DocsCommand,
    },
    Quality {
        #[command(subcommand)]
        command: QualityCommand,
    },
    Probe {
        #[command(subcommand)]
        command: ProbeCommand,
    },
}

#[derive(Subcommand)]
enum DocsCommand {
    ValidateTopology,
}

#[derive(Subcommand)]
enum QualityCommand {
    CheckLines,
}

#[derive(Subcommand)]
enum ProbeCommand {
    Status {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    LoginPlay {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    MultiplayerMutation {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    PersistPlace {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    PersistCheck {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    ProfileReconnect {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    ChunkStream {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    SurvivalItem {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    InventorySync {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    ItemPickup {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    SmpCommands {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("lkjmcrs=info")
        .with_target(false)
        .init();

    match Cli::parse().command {
        Command::Serve => crate::net::serve(Config::from_default_path()?).await?,
        Command::Docs { command } => match command {
            DocsCommand::ValidateTopology => quality::validate_docs_topology()?,
        },
        Command::Quality { command } => match command {
            QualityCommand::CheckLines => quality::check_lines()?,
        },
        Command::Probe { command } => match command {
            ProbeCommand::Status { host } => crate::probe::status(&host).await?,
            ProbeCommand::LoginPlay { host } => crate::probe::login_play(&host).await?,
            ProbeCommand::MultiplayerMutation { host } => {
                crate::probe::multiplayer_mutation(&host).await?
            }
            ProbeCommand::PersistPlace { host } => crate::probe::persist_place(&host).await?,
            ProbeCommand::PersistCheck { host } => crate::probe::persist_check(&host).await?,
            ProbeCommand::ProfileReconnect { host } => {
                crate::probe::profile_reconnect(&host).await?
            }
            ProbeCommand::ChunkStream { host } => crate::probe::chunk_stream(&host).await?,
            ProbeCommand::SurvivalItem { host } => crate::probe::survival_item(&host).await?,
            ProbeCommand::InventorySync { host } => crate::probe::inventory_sync(&host).await?,
            ProbeCommand::ItemPickup { host } => crate::probe::item_pickup(&host).await?,
            ProbeCommand::SmpCommands { host } => crate::probe::smp_commands(&host).await?,
        },
    }

    Ok(())
}
