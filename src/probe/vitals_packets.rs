use crate::probe::ProbeError;
use crate::protocol::{codec, ids};
use tokio::io::AsyncRead;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthState {
    pub health: f32,
    pub food: i32,
    pub saturation: f32,
}

pub async fn expect_update_health<S>(
    stream: &mut S,
) -> Result<HealthState, Box<dyn std::error::Error>>
where
    S: AsyncRead + Unpin,
{
    let packet = super::expect(stream, ids::play::UPDATE_HEALTH, "update health").await?;
    decode_update_health(packet.data)
}

pub fn decode_update_health(data: Vec<u8>) -> Result<HealthState, Box<dyn std::error::Error>> {
    let mut cursor = std::io::Cursor::new(data);
    let state = HealthState {
        health: codec::read_f32(&mut cursor)?,
        food: codec::read_var_i32(&mut cursor)?,
        saturation: codec::read_f32(&mut cursor)?,
    };
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err(Box::new(ProbeError::Phase("update health trailing bytes")));
    }
    Ok(state)
}

pub fn validate_health(
    state: HealthState,
    health: f32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    if (state.health - health).abs() > f32::EPSILON || state.food != 20 {
        return Err(Box::new(std::io::Error::other(format!(
            "{phase}: got health {} food {}",
            state.health, state.food
        ))));
    }
    Ok(())
}

pub fn validate_state(
    state: HealthState,
    health: f32,
    food: i32,
    saturation: f32,
    phase: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    if (state.health - health).abs() > f32::EPSILON
        || state.food != food
        || (state.saturation - saturation).abs() > f32::EPSILON
    {
        return Err(Box::new(std::io::Error::other(format!(
            "{phase}: got health {} food {} saturation {}",
            state.health, state.food, state.saturation
        ))));
    }
    Ok(())
}
