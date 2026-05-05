# Movement Flags Byte

## Report

On `2026-05-05`, a stock client disconnected during play with server log:

- phase: `play`,
- error: `packet ended early`,
- client reason: `Disconnected`.

## Cause

The server decoded `move_player_*` packets with two trailing booleans. Protocol
`774` movement packets instead use one unsigned flags byte:

- bit `0x01`: on ground,
- bit `0x02`: horizontal collision.

A vanilla `move_player_status_only` packet can contain only that one byte, so
the second boolean read reached the end of the packet.

## Fix

1. Decode all serverbound movement packets with one flags byte.
2. Update the smoke probe movement payload to send the vanilla flags byte.
3. Keep tests for trailing-byte rejection and status-only flag decoding.
