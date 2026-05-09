use crate::player::{PlayerPosition, PlayerProfile};
use crate::protocol::movement::Movement;
use crate::protocol::play::Bootstrap;
use crate::session::mining::ActiveMining;
use crate::session::registry::SessionId;
use tokio::time::{Duration, Instant};

const KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Keepalive {
    id: i64,
    sent_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct KeepaliveState {
    pending: Option<Keepalive>,
}

impl KeepaliveState {
    fn new() -> Self {
        Self { pending: None }
    }

    fn record_sent(&mut self, id: i64, sent_at: Instant) {
        self.pending = Some(Keepalive { id, sent_at });
    }

    fn ack(&mut self, id: i64) -> bool {
        match self.pending {
            Some(keepalive) if keepalive.id == id => {
                self.pending = None;
                true
            }
            _ => false,
        }
    }

    fn timed_out(&self, now: Instant) -> bool {
        self.pending
            .is_some_and(|keepalive| now.duration_since(keepalive.sent_at) >= KEEPALIVE_TIMEOUT)
    }

    fn pending_id(&self) -> Option<i64> {
        self.pending.map(|keepalive| keepalive.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaySession {
    pub id: SessionId,
    pub is_op: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
    pub horizontal_collision: bool,
    keepalive_state: KeepaliveState,
    pub age: i64,
    pub day_time: i64,
    pub dead: bool,
    pub(super) active_mining: Option<ActiveMining>,
}

impl PlaySession {
    pub fn new(bootstrap: Bootstrap, id: SessionId, is_op: bool) -> Self {
        Self {
            id,
            is_op,
            x: bootstrap.player_x,
            y: bootstrap.player_y,
            z: bootstrap.player_z,
            yaw: bootstrap.yaw,
            pitch: bootstrap.pitch,
            on_ground: false,
            horizontal_collision: false,
            keepalive_state: KeepaliveState::new(),
            age: 0,
            day_time: 0,
            dead: false,
            active_mining: None,
        }
    }

    pub fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Position {
                x,
                y,
                z,
                on_ground,
                horizontal_collision,
            } => self.update_position(x, y, z, on_ground, horizontal_collision),
            Movement::PositionLook {
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
                horizontal_collision,
            } => {
                self.update_position(x, y, z, on_ground, horizontal_collision);
                self.update_look(yaw, pitch, on_ground, horizontal_collision);
            }
            Movement::Look {
                yaw,
                pitch,
                on_ground,
                horizontal_collision,
            } => self.update_look(yaw, pitch, on_ground, horizontal_collision),
            Movement::Flying {
                on_ground,
                horizontal_collision,
            } => self.update_flags(on_ground, horizontal_collision),
        }
    }

    pub fn record_keepalive_sent(&mut self, id: i64, sent_at: Instant) {
        self.keepalive_state.record_sent(id, sent_at);
    }

    pub fn keepalive_matches(&mut self, id: i64) -> bool {
        self.keepalive_state.ack(id)
    }

    pub fn keepalive_timed_out(&self, now: Instant) -> bool {
        self.keepalive_state.timed_out(now)
    }

    pub fn keepalive_id(&self) -> Option<i64> {
        self.keepalive_state.pending_id()
    }

    pub fn advance_time(&mut self, ticks: i64) {
        self.age += ticks;
        self.day_time += ticks;
    }

    pub fn write_profile(self, profile: &mut PlayerProfile) {
        self.copy_position_to_profile(profile);
    }

    pub fn copy_position_to_profile(&self, profile: &mut PlayerProfile) {
        profile.position = PlayerPosition {
            x: self.x,
            y: self.y,
            z: self.z,
            yaw: self.yaw,
            pitch: self.pitch,
        };
    }

    pub fn move_to_spawn(&mut self) {
        self.x = 0.5;
        self.y = 80.0;
        self.z = 0.5;
        self.yaw = 0.0;
        self.pitch = 0.0;
    }

    fn update_position(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        horizontal_collision: bool,
    ) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.update_flags(on_ground, horizontal_collision);
    }

    fn update_look(&mut self, yaw: f32, pitch: f32, on_ground: bool, horizontal_collision: bool) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.update_flags(on_ground, horizontal_collision);
    }

    fn update_flags(&mut self, on_ground: bool, horizontal_collision: bool) {
        self.on_ground = on_ground;
        self.horizontal_collision = horizontal_collision;
    }
}
