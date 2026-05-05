# Packet Scope

## Implemented First

- Handshake to status.
- Handshake to login.
- Status request and JSON response.
- Ping request and pong response.
- Login hello from client.
- Login success from server.
- Login acknowledged from client.
- Configuration client settings accepted.
- Known pack selection for `minecraft:core`.
- Minimal registry data for overworld, plains, damage type, required variants,
  and timeline.
- Tags for declared dynamic registries.
- Enabled features with `minecraft:vanilla`.
- Finish configuration from server.
- Finish configuration from client.
- Play login/join packet with `minecraft:overworld` spawn info.
- Game event `13`, start waiting for level chunks.
- Chunk cache center and radius.
- Flat `level_chunk_with_light` chunks around spawn.
- Chunk batch start and finish.
- Light data for spawn chunks.
- Default spawn position.
- Time update.
- Player abilities.
- Initial player position sync.
- Keepalive serverbound and clientbound in play.
- Basic movement packets decoded from client and stored on the session.
- Creative-style block placement and breaking in loaded spawn chunks.
- Block prediction acknowledgements and single-block updates.
- Single-block updates fanned out to subscribed play sessions.

## Deferred

- Compression.
- Encryption.
- Online-mode session verification.
- Chat signing.
- Full vanilla registry coverage beyond evidence-driven first milestone
  entries.
- Persistent chunk storage and full mutable chunk resend packets.
- Inventory-backed item use.
- Resource pack negotiation.
- Complete play packet set.

## Current Vanilla Boundary

The server-list status path is vanilla-shaped for `1.21.11`.
The login path reaches configuration, negotiates the vanilla core pack,
loads the required non-empty registries, enters play, sends game event `13`,
and sends a deterministic `5x5` flat spawn chunk batch for advertised radius
`2`. A player join was reported on `2026-05-05` after the `0x26` fix. Treat
that as manual success evidence without raw logs attached.

Dynamic registries are intentionally minimal and evidence-driven. They are not
full vanilla coverage. `minecraft:damage_type` is required before play login can
construct the client level because vanilla play login constructs its built-in
damage sources at that point.

The target remains stock Minecraft Java Edition `1.21.11`. Fabric or modded
client reports may be recorded only when they expose a vanilla protocol gap.
The active manual boundary is owned by
[../../operations/verification/join-boundary.md](../../operations/verification/join-boundary.md).

## Next Join Sequence

1. Client sends handshake with protocol `774` and next state `login`.
2. Client sends login start.
3. Server sends login success.
4. Client sends login acknowledged.
5. Client may send configuration settings.
6. Server sends known packs with `minecraft:core` version `1.21.11`.
7. Client replies with selected known packs.
8. Server sends registry data and tags.
9. Server sends enabled features with `minecraft:vanilla`.
10. Server sends finish configuration.
11. Client acknowledges finish configuration.
12. Server sends play login, game event `13`, the full advertised flat chunk
    radius, light, position, and keepalive.

## Rule

Every implemented packet must have encode or decode tests for boundary values
where practical.
