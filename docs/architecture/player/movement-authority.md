# Movement Authority

## Goal

Define the current trust boundary for serverbound movement before full
survival physics and anti-cheat rules exist.

## Current Authority

- The client may send vanilla-shaped movement packets in play state.
- The server decodes movement payloads and rejects malformed packet shapes.
- Accepted movement updates session-local position, look, on-ground, and
  horizontal-collision state.
- The server persists the final session position on disconnect.
- Movement may change the chunk-streaming center and chunk subscriptions.

## Not Yet Authoritative

- The server does not yet simulate collision resolution for player movement.
- The server does not broadcast player movement to other sessions.
- Movement does not directly mutate world chunks.
- Teleport safety scans are not implemented for homes, warps, or respawn.
- Anti-cheat speed, reach, fall, and flight checks are out of scope.

## Target Authority

- Reject non-finite coordinates and coordinates outside world height bounds.
- Reject movement that exceeds documented speed or acceleration envelopes.
- Reject no-clip movement through authoritative loaded blocks.
- Rate-limit movement and interaction packets per session.
- Send corrective position packets after rejection and persist only accepted
  positions.
- Keep all block mutation authority in region actors.

## Packet Shape

- `0x1d position`: position plus one flags byte.
- `0x1e position_look`: position, look, plus one flags byte.
- `0x1f look`: look plus one flags byte.
- `0x20 status_only`: one flags byte.
- Flags bit `0x01` means on ground.
- Flags bit `0x02` means horizontal collision.
- Trailing movement bytes are invalid.

## Transition Rules

1. Add server-side physics as narrow checks with compose or unit evidence.
2. Keep chunk streaming driven by the accepted session position.
3. Keep world mutation authority in region actors, not movement decoding.
4. Document any new disconnect reason before adding it to runtime behavior.
