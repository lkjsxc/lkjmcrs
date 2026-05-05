#[derive(Debug, thiserror::Error)]
pub enum RegionActorError {
    #[error("region actor is closed")]
    Closed,
}
