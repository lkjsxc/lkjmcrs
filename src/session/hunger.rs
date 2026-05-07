use crate::player::{GameMode, PlayerProfile};
use crate::session::play_state::PlaySession;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HungerTick {
    None,
    Changed,
    Starve,
}

pub fn tick(profile: &mut PlayerProfile, session: &PlaySession) -> HungerTick {
    if session.dead || profile.game_mode == GameMode::Creative {
        return HungerTick::None;
    }
    if profile.vitals.health < 20.0 && profile.vitals.hunger >= 18 {
        profile.vitals.health = (profile.vitals.health + 1.0).min(20.0);
        spend_regen_cost(profile);
        return HungerTick::Changed;
    }
    if profile.vitals.saturation > 0.0 {
        profile.vitals.saturation = (profile.vitals.saturation - 0.5).max(0.0);
        return HungerTick::Changed;
    }
    if profile.vitals.hunger > 0 {
        profile.vitals.hunger -= 1;
        return HungerTick::Changed;
    }
    HungerTick::Starve
}

fn spend_regen_cost(profile: &mut PlayerProfile) {
    if profile.vitals.saturation > 0.0 {
        profile.vitals.saturation = (profile.vitals.saturation - 1.0).max(0.0);
    } else if profile.vitals.hunger > 0 {
        profile.vitals.hunger -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{HungerTick, tick};
    use crate::player::{GameMode, PlayerProfile};
    use crate::protocol::play;
    use crate::session::play_state::PlaySession;
    use crate::session::registry::SessionId;
    use uuid::Uuid;

    #[test]
    fn regen_spends_saturation() {
        let mut profile = PlayerProfile::new(Uuid::from_u128(1), "A");
        profile.vitals.health = 19.0;
        profile.vitals.saturation = 1.0;
        let session = session();

        assert_eq!(tick(&mut profile, &session), HungerTick::Changed);
        assert_eq!(profile.vitals.health, 20.0);
        assert_eq!(profile.vitals.saturation, 0.0);
    }

    #[test]
    fn starvation_is_reported_without_direct_damage() {
        let mut profile = PlayerProfile::new(Uuid::from_u128(2), "B");
        profile.vitals.hunger = 0;
        profile.vitals.saturation = 0.0;
        let session = session();

        assert_eq!(tick(&mut profile, &session), HungerTick::Starve);
        assert_eq!(profile.vitals.health, 20.0);
    }

    #[test]
    fn creative_does_not_drain() {
        let mut profile = PlayerProfile::new(Uuid::from_u128(3), "C");
        profile.game_mode = GameMode::Creative;
        let session = session();

        assert_eq!(tick(&mut profile, &session), HungerTick::None);
        assert_eq!(profile.vitals.hunger, 20);
    }

    fn session() -> PlaySession {
        PlaySession::new(play::Bootstrap::new(100), SessionId(1), false)
    }
}
