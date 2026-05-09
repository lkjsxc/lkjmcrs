use crate::config::Config;
use crate::player::{PlayerDefaults, PlayerPosition};
use crate::session::handler::ServerContext;

pub(super) fn player_defaults(context: &ServerContext) -> PlayerDefaults {
    PlayerDefaults {
        game_mode: context.config.default_game_mode,
        position: context.spawn_position,
    }
}

pub(super) fn player_spawn(spawn: (f64, f64, f64)) -> PlayerPosition {
    PlayerPosition {
        x: spawn.0,
        y: spawn.1,
        z: spawn.2,
        yaw: 0.0,
        pitch: 0.0,
    }
}

pub(super) fn play_settings(
    config: &Config,
    spawn_position: PlayerPosition,
) -> crate::session::play::PlaySettings {
    crate::session::play::PlaySettings {
        max_players: config.max_players,
        view_distance: config.view_distance,
        simulation_distance: config.simulation_distance,
        spawn_position,
    }
}
