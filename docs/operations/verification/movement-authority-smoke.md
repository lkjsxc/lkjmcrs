# Movement Authority Smoke

## Goal

Verify the current movement trust boundary over the public play protocol.

## Required Scenario

1. Start a live server through Docker Compose.
2. Join with a first-party probe client.
3. Send accepted movement and look packet shapes.
4. Send non-finite and out-of-height movement and require correction packets.
5. Cross a chunk-center boundary and verify chunk streaming follows the
   accepted position.
6. Reconnect and verify the last accepted position persisted.

## Assertions

- Movement uses one flags byte for on-ground and horizontal-collision state.
- Accepted movement does not create block mutations.
- Rejected movement does not replace the persisted accepted position.
- Movement-driven chunk streaming stays bounded by the configured radius and
  follow-up batch limits.
- Final position persists across reconnect when the reconnect probe covers the
  same movement shape.

## Gate Command

- Compose service: `movement-authority`.
- It complements [smoke-probe.md](smoke-probe.md),
  [profile-reconnect-smoke.md](profile-reconnect-smoke.md),
  [chunk-stream-smoke.md](chunk-stream-smoke.md), and
  [render-distance-smoke.md](render-distance-smoke.md).
