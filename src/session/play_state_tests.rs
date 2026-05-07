use crate::protocol::movement::Movement;
use crate::protocol::play::Bootstrap;
use crate::session::play_state::PlaySession;
use crate::session::registry::SessionId;
use tokio::time::{Duration, Instant};

#[test]
fn movement_updates_session_local_state() {
    let mut session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);
    session.apply_movement(Movement::PositionLook {
        x: 2.0,
        y: 81.0,
        z: -3.0,
        yaw: 45.0,
        pitch: 15.0,
        on_ground: true,
        horizontal_collision: false,
    });

    assert_eq!(session.x, 2.0);
    assert_eq!(session.y, 81.0);
    assert_eq!(session.z, -3.0);
    assert_eq!(session.yaw, 45.0);
    assert!(session.on_ground);
}

#[test]
fn time_advances_by_ticks() {
    let mut session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);
    session.advance_time(20);
    assert_eq!(session.age, 20);
    assert_eq!(session.day_time, 20);
}

#[test]
fn matching_keepalive_clears_pending_id() {
    let mut session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);
    let sent_at = Instant::now();

    session.record_keepalive_sent(123, sent_at);
    assert_eq!(session.keepalive_id(), Some(123));
    assert!(session.keepalive_matches(123));
    assert_eq!(session.keepalive_id(), None);
}

#[test]
fn mismatched_keepalive_does_not_clear_pending_id() {
    let mut session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);

    session.record_keepalive_sent(123, Instant::now());
    assert!(!session.keepalive_matches(456));
    assert_eq!(session.keepalive_id(), Some(123));
}

#[test]
fn keepalive_times_out_after_30_seconds() {
    let mut session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);
    let sent_at = Instant::now();

    session.record_keepalive_sent(123, sent_at);
    assert!(!session.keepalive_timed_out(sent_at + Duration::from_secs(29)));
    assert!(session.keepalive_timed_out(sent_at + Duration::from_secs(30)));
}
