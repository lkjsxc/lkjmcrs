use crate::session::play_state::PlaySession;
use crate::world::BlockPos;

const PLAYER_EYE_HEIGHT: f64 = 1.62;
const MAX_BLOCK_REACH: f64 = 6.0;

pub fn can_reach_block(session: &PlaySession, pos: BlockPos) -> bool {
    let dx = block_center(pos.x) - session.x;
    let dy = block_center(pos.y) - (session.y + PLAYER_EYE_HEIGHT);
    let dz = block_center(pos.z) - session.z;
    let distance_squared = dx.mul_add(dx, dy.mul_add(dy, dz * dz));
    distance_squared <= MAX_BLOCK_REACH * MAX_BLOCK_REACH
}

fn block_center(value: i32) -> f64 {
    f64::from(value) + 0.5
}

#[cfg(test)]
mod tests {
    use super::can_reach_block;
    use crate::protocol::play::Bootstrap;
    use crate::session::play_state::PlaySession;
    use crate::session::registry::SessionId;
    use crate::world::BlockPos;

    #[test]
    fn nearby_block_is_reachable() {
        let session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);

        assert!(can_reach_block(&session, BlockPos::new(0, 80, 0)));
    }

    #[test]
    fn distant_block_is_not_reachable() {
        let session = PlaySession::new(Bootstrap::new(100), SessionId(1), false);

        assert!(!can_reach_block(&session, BlockPos::new(8, 80, 0)));
    }
}
