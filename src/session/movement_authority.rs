use crate::protocol::movement::Movement;
use crate::session::play_state::PlaySession;
use crate::world::{MAX_Y, MIN_Y};

const MAX_SINGLE_PACKET_DELTA: f64 = 128.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MovementRejection {
    pub reason: &'static str,
}

pub(super) fn validate(session: &PlaySession, movement: Movement) -> Result<(), MovementRejection> {
    let Some((x, y, z)) = movement.position() else {
        return Ok(());
    };
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return reject("non-finite movement");
    }
    if y < f64::from(MIN_Y) || y > f64::from(MAX_Y + 2) {
        return reject("movement outside world height");
    }
    let dx = (x - session.x).abs();
    let dy = (y - session.y).abs();
    let dz = (z - session.z).abs();
    if dx.max(dy).max(dz) > MAX_SINGLE_PACKET_DELTA {
        return reject("movement delta too large");
    }
    Ok(())
}

fn reject(reason: &'static str) -> Result<(), MovementRejection> {
    Err(MovementRejection { reason })
}

trait MovementPosition {
    fn position(self) -> Option<(f64, f64, f64)>;
}

impl MovementPosition for Movement {
    fn position(self) -> Option<(f64, f64, f64)> {
        match self {
            Self::Position { x, y, z, .. } | Self::PositionLook { x, y, z, .. } => Some((x, y, z)),
            Self::Look { .. } | Self::Flying { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::protocol::movement::Movement;
    use crate::protocol::play::Bootstrap;
    use crate::session::play_state::PlaySession;
    use crate::session::registry::SessionId;

    #[test]
    fn rejects_non_finite_position() {
        let session = PlaySession::new(Bootstrap::new(10), SessionId(1), false);
        let movement = Movement::Position {
            x: f64::NAN,
            y: 80.0,
            z: 0.5,
            on_ground: true,
            horizontal_collision: false,
        };

        assert!(validate(&session, movement).is_err());
    }

    #[test]
    fn allows_probe_scale_chunk_jump() {
        let session = PlaySession::new(Bootstrap::new(10), SessionId(1), false);
        let movement = Movement::Position {
            x: 44.5,
            y: 80.0,
            z: 0.5,
            on_ground: true,
            horizontal_collision: false,
        };

        assert!(validate(&session, movement).is_ok());
    }
}
