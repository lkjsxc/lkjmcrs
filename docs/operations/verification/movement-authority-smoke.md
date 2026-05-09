# Movement Authority Smoke

## Goal

Verify the current movement trust boundary over the public play protocol.

## Required Scenario

1. Start a live server through Docker Compose.
2. Join with a first-party probe client.
3. Send each accepted movement packet shape.
4. Verify session-local position and look changes through observable effects.
5. Cross a chunk-center boundary and verify chunk streaming follows the
   accepted position.
6. Send malformed movement with trailing bytes and verify the connection closes
   through the normal error path.

## Assertions

- Movement uses one flags byte for on-ground and horizontal-collision state.
- Accepted movement does not create block mutations.
- Movement-driven chunk streaming stays bounded by the configured radius and
  follow-up batch limits.
- Final position persists across reconnect when the reconnect probe covers the
  same movement shape.

## Deferred Command

Current movement coverage is split across
[smoke-probe.md](smoke-probe.md), [profile-reconnect-smoke.md](profile-reconnect-smoke.md),
[chunk-stream-smoke.md](chunk-stream-smoke.md), and
[render-distance-smoke.md](render-distance-smoke.md). A single movement
authority command is not active yet.
