use crate::scheduler::RegionActorError;
use crate::scheduler::region_command::RegionCommand;
use crate::world::{ChunkPos, ChunkSnapshot, WorldStorage};
use std::time::Instant;
use tokio::sync::mpsc;

pub(super) fn load_chunks(
    id: u64,
    storage: WorldStorage,
    positions: Vec<ChunkPos>,
    outbox: mpsc::Sender<RegionCommand>,
) {
    tokio::spawn(async move {
        let count = positions.len();
        let started = Instant::now();
        let result = tokio::task::spawn_blocking(move || {
            positions
                .into_iter()
                .map(|pos| storage.load_chunk(pos))
                .collect::<Result<Vec<_>, _>>()
        })
        .await
        .map_err(|error| RegionActorError::StorageTask(error.to_string()))
        .and_then(|result| result.map_err(RegionActorError::from));
        tracing::info!(
            target: "lkjmcrs::scale",
            storage_operation = "load_chunks",
            storage_chunks = count,
            storage_elapsed_ms = started.elapsed().as_millis() as u64,
            storage_ok = result.is_ok(),
            "world storage job completed"
        );
        let _ = outbox
            .send(RegionCommand::LoadComplete { id, result })
            .await;
    });
}

pub(super) fn save_chunk(
    id: u64,
    storage: WorldStorage,
    chunk: ChunkSnapshot,
    outbox: mpsc::Sender<RegionCommand>,
) {
    tokio::spawn(async move {
        let started = Instant::now();
        let result = tokio::task::spawn_blocking(move || storage.save_chunk(&chunk))
            .await
            .map_err(|error| RegionActorError::StorageTask(error.to_string()))
            .and_then(|result| result.map_err(RegionActorError::from));
        tracing::info!(
            target: "lkjmcrs::scale",
            storage_operation = "save_chunk",
            storage_chunks = 1_usize,
            storage_elapsed_ms = started.elapsed().as_millis() as u64,
            storage_ok = result.is_ok(),
            "world storage job completed"
        );
        let _ = outbox
            .send(RegionCommand::SaveComplete { id, result })
            .await;
    });
}
