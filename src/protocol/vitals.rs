use crate::protocol::{codec, nbt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthUpdate {
    pub health: f32,
    pub hunger: i32,
    pub saturation: f32,
}

pub fn encode_update_health(vitals: HealthUpdate) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_f32(&mut out, vitals.health);
    codec::write_var_i32(&mut out, vitals.hunger);
    codec::write_f32(&mut out, vitals.saturation);
    out
}

pub fn encode_death_combat_event(player_id: i32, message: &str) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, player_id);
    nbt::write_anonymous_compound(
        &mut out,
        &nbt::compound(vec![("text", nbt::string(message))]),
    );
    out
}

pub fn encode_respawn_request(action_id: i32) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_var_i32(&mut out, action_id);
    out
}

#[cfg(test)]
mod tests {
    use super::{HealthUpdate, encode_death_combat_event, encode_update_health};
    use crate::protocol::codec;
    use std::io::Cursor;

    #[test]
    fn update_health_writes_protocol_payload() {
        let payload = encode_update_health(HealthUpdate {
            health: 12.5,
            hunger: 20,
            saturation: 5.0,
        });
        let mut cursor = Cursor::new(payload);
        assert_eq!(codec::read_f32(&mut cursor).unwrap(), 12.5);
        assert_eq!(codec::read_var_i32(&mut cursor).unwrap(), 20);
        assert_eq!(codec::read_f32(&mut cursor).unwrap(), 5.0);
    }

    #[test]
    fn death_event_contains_text() {
        let payload = encode_death_combat_event(1, "Target died");
        assert_eq!(payload[0], 1);
        assert!(String::from_utf8_lossy(&payload).contains("Target died"));
    }
}
