use crate::scheduler::RegionActorError;
use crate::scheduler::region_command::RegionCommand;
use crate::world::{ChunkPos, ChunkSnapshot, WorldStorage};
use tokio::sync::mpsc;

pub(super) fn load_chunks(
    id: u64,
    storage: WorldStorage,
    positions: Vec<ChunkPos>,
    outbox: mpsc::Sender<RegionCommand>,
) {
    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || {
            positions
                .into_iter()
                .map(|pos| storage.load_chunk(pos))
                .collect::<Result<Vec<_>, _>>()
        })
        .await
        .map_err(|error| RegionActorError::StorageTask(error.to_string()))
        .and_then(|result| result.map_err(RegionActorError::from));
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
        let result = tokio::task::spawn_blocking(move || storage.save_chunk(&chunk))
            .await
            .map_err(|error| RegionActorError::StorageTask(error.to_string()))
            .and_then(|result| result.map_err(RegionActorError::from));
        let _ = outbox
            .send(RegionCommand::SaveComplete { id, result })
            .await;
    });
}
