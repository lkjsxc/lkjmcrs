use tokio::time::Duration;

pub const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
pub const TIME_INTERVAL: Duration = Duration::from_secs(1);
pub const HUNGER_INTERVAL: Duration = Duration::from_secs(4);
pub const CHUNK_DRAIN_INTERVAL: Duration = Duration::from_millis(50);
pub const TIME_STEP_TICKS: i64 = 20;
