use crate::protocol::codec;
mod bootstrap;
pub use bootstrap::{Bootstrap, chunk_count_for_radius};

const OVERWORLD: &str = "minecraft:overworld";
const START_WAITING_FOR_LEVEL_CHUNKS: u8 = 13;

pub fn encode_login(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i32(&mut out, bootstrap.entity_id);
    codec::write_bool(&mut out, false);
    codec::write_var_i32(&mut out, 1);
    codec::write_string(&mut out, OVERWORLD);
    codec::write_var_i32(&mut out, bootstrap.max_players);
    codec::write_var_i32(&mut out, bootstrap.view_distance);
    codec::write_var_i32(&mut out, bootstrap.simulation_distance);
    codec::write_bool(&mut out, false);
    codec::write_bool(&mut out, true);
    codec::write_bool(&mut out, false);
    encode_spawn_info(&mut out, bootstrap);
    codec::write_bool(&mut out, false);
    out
}

pub fn encode_chunk_cache_center(chunk_x: i32, chunk_z: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, chunk_x);
    codec::write_var_i32(&mut out, chunk_z);
    out
}

pub fn encode_chunk_cache_radius(radius: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, radius);
    out
}

pub fn encode_default_spawn_position(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_string(&mut out, OVERWORLD);
    codec::write_position(
        &mut out,
        bootstrap.spawn_x,
        bootstrap.spawn_y,
        bootstrap.spawn_z,
    );
    codec::write_f32(&mut out, 0.0);
    codec::write_f32(&mut out, 0.0);
    out
}

pub fn encode_time(age: i64, time: i64) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i64(&mut out, age);
    codec::write_i64(&mut out, time);
    codec::write_bool(&mut out, true);
    out
}

pub fn encode_player_abilities() -> Vec<u8> {
    encode_player_abilities_for(Bootstrap::new(100))
}

pub fn encode_player_abilities_for(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i8(&mut out, bootstrap.ability_flags);
    codec::write_f32(&mut out, 0.05);
    codec::write_f32(&mut out, 0.1);
    out
}

pub fn encode_start_waiting_for_chunks() -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_u8(&mut out, START_WAITING_FOR_LEVEL_CHUNKS);
    codec::write_f32(&mut out, 0.0);
    out
}

pub fn encode_initial_position(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, bootstrap.teleport_id());
    codec::write_f64(&mut out, bootstrap.player_x);
    codec::write_f64(&mut out, bootstrap.player_y);
    codec::write_f64(&mut out, bootstrap.player_z);
    codec::write_f64(&mut out, 0.0);
    codec::write_f64(&mut out, 0.0);
    codec::write_f64(&mut out, 0.0);
    codec::write_f32(&mut out, bootstrap.yaw);
    codec::write_f32(&mut out, bootstrap.pitch);
    codec::write_u32(&mut out, 0);
    out
}

pub fn encode_keepalive(id: i64) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i64(&mut out, id);
    out
}

fn encode_spawn_info(out: &mut Vec<u8>, bootstrap: Bootstrap) {
    codec::write_var_i32(out, 0);
    codec::write_string(out, OVERWORLD);
    codec::write_i64(out, 0);
    codec::write_i8(out, bootstrap.game_mode);
    codec::write_u8(out, 255);
    codec::write_bool(out, false);
    codec::write_bool(out, true);
    codec::write_bool(out, false);
    codec::write_var_i32(out, 0);
    codec::write_var_i32(out, 63);
}
