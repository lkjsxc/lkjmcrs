use crate::protocol::block_interaction;
use crate::protocol::ids;
use crate::session::SessionState;
use crate::session::bootstrap::block_state_id;
use crate::session::error::ConnectionError;
use crate::session::io::write_packet;
use crate::world::{BlockPos, BlockState};
use tokio::io::AsyncWrite;

pub async fn send_prediction_ack<W>(
    writer: &mut W,
    phase: SessionState,
    sequence: i32,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::BLOCK_CHANGED_ACK,
        &block_interaction::encode_block_changed_ack(sequence),
    )
    .await
}

pub async fn send_block_update<W>(
    writer: &mut W,
    phase: SessionState,
    pos: BlockPos,
    state: BlockState,
) -> Result<(), ConnectionError>
where
    W: AsyncWrite + Unpin,
{
    write_packet(
        writer,
        phase,
        ids::play::BLOCK_UPDATE,
        &block_interaction::encode_block_update(to_wire_pos(pos), block_state_id(state)),
    )
    .await
}

fn to_wire_pos(pos: BlockPos) -> block_interaction::BlockPos {
    block_interaction::BlockPos {
        x: pos.x,
        y: pos.y,
        z: pos.z,
    }
}
