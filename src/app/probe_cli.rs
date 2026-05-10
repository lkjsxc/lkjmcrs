use clap::Subcommand;

#[derive(Subcommand)]
pub(super) enum ProbeCommand {
    LoginPlay {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    MultiplayerMutation {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    MovementAuthority {
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
    StorageSectionPersistence {
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
    ScaleChunkStream {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    TerrainGeneration {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    TerrainRivers {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    TerrainCaves {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    TerrainQuality {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    ScaleLoadMetrics {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    ScaleMovingPending {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    RenderDistance {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    RenderMovingPending {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    SurvivalItem {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
    SurvivalVitals {
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
    OnlineAuth {
        #[arg(long, default_value = "127.0.0.1:25565")]
        host: String,
    },
}

pub(super) async fn run(command: ProbeCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ProbeCommand::LoginPlay { host } => crate::probe::login_play(&host).await?,
        ProbeCommand::MultiplayerMutation { host } => {
            crate::probe::multiplayer_mutation(&host).await?
        }
        ProbeCommand::MovementAuthority { host } => crate::probe::movement_authority(&host).await?,
        ProbeCommand::PersistPlace { host } => crate::probe::persist_place(&host).await?,
        ProbeCommand::PersistCheck { host } => crate::probe::persist_check(&host).await?,
        ProbeCommand::StorageSectionPersistence { host } => {
            crate::probe::storage_section_persistence(&host).await?
        }
        ProbeCommand::ProfileReconnect { host } => crate::probe::profile_reconnect(&host).await?,
        ProbeCommand::ChunkStream { host } => crate::probe::chunk_stream(&host).await?,
        ProbeCommand::ScaleChunkStream { host } => crate::probe::scale_chunk_stream(&host).await?,
        ProbeCommand::TerrainGeneration { host } => crate::probe::terrain_generation(&host).await?,
        ProbeCommand::TerrainRivers { host } => crate::probe::terrain_rivers(&host).await?,
        ProbeCommand::TerrainCaves { host } => crate::probe::terrain_caves(&host).await?,
        ProbeCommand::TerrainQuality { host } => crate::probe::terrain_quality(&host).await?,
        ProbeCommand::ScaleLoadMetrics { host } => crate::probe::scale_load_metrics(&host).await?,
        ProbeCommand::ScaleMovingPending { host } => {
            crate::probe::scale_moving_pending(&host).await?
        }
        ProbeCommand::RenderDistance { host } => crate::probe::render_distance(&host).await?,
        ProbeCommand::RenderMovingPending { host } => {
            crate::probe::render_moving_pending(&host).await?
        }
        ProbeCommand::SurvivalItem { host } => crate::probe::survival_item(&host).await?,
        ProbeCommand::SurvivalVitals { host } => crate::probe::survival_vitals(&host).await?,
        ProbeCommand::InventorySync { host } => crate::probe::inventory_sync(&host).await?,
        ProbeCommand::ItemPickup { host } => crate::probe::item_pickup(&host).await?,
        ProbeCommand::SmpCommands { host } => crate::probe::smp_commands(&host).await?,
        ProbeCommand::OnlineAuth { host } => crate::probe::online_auth(&host).await?,
    }
    Ok(())
}
