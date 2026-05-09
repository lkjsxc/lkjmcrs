use crate::player::GameMode;
use crate::world::{BlockPos, BlockState};
use tokio::time::{Duration, Instant};

pub(super) const DIRT_BREAK_TIME: Duration = Duration::from_millis(750);
pub(super) const STONE_BREAK_TIME: Duration = Duration::from_millis(1500);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ActiveMining {
    pub pos: BlockPos,
    pub ready_at: Instant,
}

impl crate::session::play_state::PlaySession {
    pub(super) fn start_mining(&mut self, pos: BlockPos, state: BlockState, now: Instant) {
        self.active_mining = Some(ActiveMining {
            pos,
            ready_at: now + required_break_time(state, crate::player::GameMode::Survival),
        });
    }

    pub(super) fn stop_mining(&mut self, pos: BlockPos, now: Instant) -> bool {
        let Some(active) = self.active_mining else {
            return false;
        };
        self.active_mining = None;
        active.pos == pos && now >= active.ready_at
    }

    pub(super) fn abort_mining(&mut self) {
        self.active_mining = None;
    }
}

pub(super) fn required_break_time(state: BlockState, game_mode: GameMode) -> Duration {
    if game_mode == GameMode::Creative {
        return Duration::ZERO;
    }
    match state {
        BlockState::Dirt | BlockState::GrassBlock => DIRT_BREAK_TIME,
        BlockState::Stone => STONE_BREAK_TIME,
        BlockState::Air | BlockState::Bedrock | BlockState::Water => Duration::ZERO,
    }
}

pub(super) fn can_start_mining(state: BlockState) -> bool {
    !matches!(
        state,
        BlockState::Air | BlockState::Bedrock | BlockState::Water
    )
}

#[cfg(test)]
mod tests {
    use super::{can_start_mining, required_break_time};
    use crate::player::GameMode;
    use crate::world::BlockState;
    use tokio::time::Duration;

    #[test]
    fn survival_break_times_match_contract() {
        assert_eq!(
            required_break_time(BlockState::GrassBlock, GameMode::Survival),
            Duration::from_millis(750)
        );
        assert_eq!(
            required_break_time(BlockState::Stone, GameMode::Survival),
            Duration::from_millis(1500)
        );
    }

    #[test]
    fn creative_breaks_immediately() {
        assert_eq!(
            required_break_time(BlockState::Stone, GameMode::Creative),
            Duration::ZERO
        );
    }

    #[test]
    fn air_and_bedrock_do_not_start_mining() {
        assert!(!can_start_mining(BlockState::Air));
        assert!(!can_start_mining(BlockState::Bedrock));
        assert!(!can_start_mining(BlockState::Water));
        assert!(can_start_mining(BlockState::Dirt));
    }
}
