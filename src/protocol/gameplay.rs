use crate::protocol::codec;

const CHANGE_GAME_MODE_EVENT: u8 = 3;

pub fn encode_game_mode_change(game_mode: i8) -> Vec<u8> {
    let mut out = Vec::new();
    codec::write_u8(&mut out, CHANGE_GAME_MODE_EVENT);
    codec::write_f32(&mut out, f32::from(game_mode));
    out
}

#[cfg(test)]
mod tests {
    use super::encode_game_mode_change;

    #[test]
    fn game_mode_change_uses_event_three() {
        assert_eq!(encode_game_mode_change(1)[0], 3);
    }
}
