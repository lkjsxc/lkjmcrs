use crate::protocol::play::{
    Bootstrap, encode_chunk_cache_center, encode_chunk_cache_radius, encode_default_spawn_position,
    encode_initial_position, encode_login, encode_player_abilities,
    encode_start_waiting_for_chunks, encode_time,
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
    let bootstrap = Bootstrap::new(100);
    assert_eq!(encode_chunk_cache_radius(bootstrap.view_distance), vec![2]);
    assert_eq!(bootstrap.chunk_count(), 25);
}

#[test]
fn spawn_position_encodes_global_position() {
    let payload = encode_default_spawn_position(Bootstrap::new(100));
    assert_eq!(&payload[..20], b"\x13minecraft:overworld");
    assert_eq!(&payload[20..28], &[0, 0, 0, 0, 0, 0, 0, 80]);
}

#[test]
fn time_abilities_and_game_state_change_are_stable() {
    assert_eq!(
        encode_time(0, 0),
        vec![0; 16].into_iter().chain([1]).collect::<Vec<_>>()
    );
    assert_eq!(encode_player_abilities()[0], 0x0d);
    assert_eq!(encode_start_waiting_for_chunks(), vec![13, 0, 0, 0, 0]);
}

#[test]
fn initial_position_contains_teleport_id() {
    let payload = encode_initial_position(Bootstrap::new(100));
    assert_eq!(payload[0], 1);
    assert_eq!(payload.len(), 61);
}
