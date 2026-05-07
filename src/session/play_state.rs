use crate::player::{PlayerPosition, PlayerProfile};
use crate::protocol::movement::Movement;
use crate::protocol::play::Bootstrap;
use crate::session::registry::SessionId;

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
    pub last_keepalive_id: i64,
    pub age: i64,
    pub day_time: i64,
    pub dead: bool,
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
            last_keepalive_id: 0,
            age: 0,
            day_time: 0,
            dead: false,
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

    pub fn record_keepalive_sent(&mut self, id: i64) {
        self.last_keepalive_id = id;
    }

    pub fn keepalive_matches(&self, id: i64) -> bool {
        self.last_keepalive_id == id
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

#[cfg(test)]
mod tests {
    use super::PlaySession;
    use crate::protocol::movement::Movement;
    use crate::protocol::play::Bootstrap;
    use crate::session::registry::SessionId;

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
}
