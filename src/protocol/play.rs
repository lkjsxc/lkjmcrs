use crate::protocol::codec;

const OVERWORLD: &str = "minecraft:overworld";
const TELEPORT_ID: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bootstrap {
    pub entity_id: i32,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
}

impl Bootstrap {
    pub fn new(max_players: usize) -> Self {
        Self {
            entity_id: 1,
            max_players: max_players as i32,
            view_distance: 2,
            simulation_distance: 2,
            spawn_x: 0,
            spawn_y: 80,
            spawn_z: 0,
        }
    }

    pub const fn teleport_id(self) -> i32 {
        TELEPORT_ID
    }
}

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
    encode_spawn_info(&mut out);
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
    let mut out = Vec::new();
    codec::write_i8(&mut out, 0x0d);
    codec::write_f32(&mut out, 0.05);
    codec::write_f32(&mut out, 0.1);
    out
}

pub fn encode_initial_position(bootstrap: Bootstrap) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, bootstrap.teleport_id());
    codec::write_f64(&mut out, f64::from(bootstrap.spawn_x) + 0.5);
    codec::write_f64(&mut out, f64::from(bootstrap.spawn_y));
    codec::write_f64(&mut out, f64::from(bootstrap.spawn_z) + 0.5);
    codec::write_f64(&mut out, 0.0);
    codec::write_f64(&mut out, 0.0);
    codec::write_f64(&mut out, 0.0);
    codec::write_f32(&mut out, 0.0);
    codec::write_f32(&mut out, 0.0);
    codec::write_u32(&mut out, 0);
    out
}

pub fn encode_keepalive(id: i64) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_i64(&mut out, id);
    out
}

fn encode_spawn_info(out: &mut Vec<u8>) {
    codec::write_var_i32(out, 0);
    codec::write_string(out, OVERWORLD);
    codec::write_i64(out, 0);
    codec::write_i8(out, 1);
    codec::write_u8(out, 255);
    codec::write_bool(out, false);
    codec::write_bool(out, true);
    codec::write_bool(out, false);
    codec::write_var_i32(out, 0);
    codec::write_var_i32(out, 63);
}

#[cfg(test)]
mod tests {
    use super::{
        Bootstrap, encode_chunk_cache_center, encode_chunk_cache_radius,
        encode_default_spawn_position, encode_initial_position, encode_login,
        encode_player_abilities, encode_time,
    };

    #[test]
    fn login_packet_has_stable_prefix() {
        let payload = encode_login(Bootstrap::new(100));
        assert_eq!(
            &payload[..29],
            b"\0\0\0\x01\0\x01\x13minecraft:overworldd\x02\x02"
        );
    }

    #[test]
    fn chunk_cache_packets_are_varints() {
        assert_eq!(encode_chunk_cache_center(0, 0), vec![0, 0]);
        assert_eq!(encode_chunk_cache_radius(2), vec![2]);
    }

    #[test]
    fn spawn_position_encodes_global_position() {
        let payload = encode_default_spawn_position(Bootstrap::new(100));
        assert_eq!(&payload[..20], b"\x13minecraft:overworld");
        assert_eq!(&payload[20..28], &[0, 0, 0, 0, 0, 0, 0, 80]);
    }

    #[test]
    fn time_and_abilities_are_stable() {
        assert_eq!(
            encode_time(0, 0),
            vec![0; 16].into_iter().chain([1]).collect::<Vec<_>>()
        );
        assert_eq!(encode_player_abilities()[0], 0x0d);
    }

    #[test]
    fn initial_position_contains_teleport_id() {
        let payload = encode_initial_position(Bootstrap::new(100));
        assert_eq!(payload[0], 1);
        assert_eq!(payload.len(), 61);
    }
}
