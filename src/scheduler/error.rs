#[derive(Debug, thiserror::Error)]
pub enum RegionActorError {
    #[error("region actor is closed")]
    Closed,
    #[error(transparent)]
    Storage(#[from] crate::world::WorldStorageError),
}
