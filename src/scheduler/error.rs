#[derive(Debug, thiserror::Error)]
pub enum RegionActorError {
    #[error("region actor is closed")]
    Closed,
    #[error(transparent)]
    Storage(#[from] crate::world::WorldStorageError),
    #[error("storage task failed: {0}")]
    StorageTask(String),
    #[error("loaded chunk {0},{1} is missing from region memory")]
    MissingLoadedChunk(i32, i32),
}
