use crate::player::PlayerProfile;
use crate::protocol::play;
use crate::session::play::PlaySettings;

pub fn from_profile(settings: PlaySettings, profile: &PlayerProfile) -> play::Bootstrap {
    play::Bootstrap::new(settings.max_players)
        .with_distances(settings.view_distance, settings.simulation_distance)
        .with_player_state(
            (profile.position.x, profile.position.y, profile.position.z),
            (profile.position.yaw, profile.position.pitch),
            (
                profile.game_mode.vanilla_id(),
                profile.game_mode.ability_flags(),
            ),
        )
}
