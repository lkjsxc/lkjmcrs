use tokio::time::{self, Duration, Instant, Interval, MissedTickBehavior};

pub fn delayed_interval(duration: Duration) -> Interval {
    let mut interval = time::interval_at(Instant::now() + duration, duration);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    interval
}
