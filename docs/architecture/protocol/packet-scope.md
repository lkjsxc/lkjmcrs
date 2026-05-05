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
- Chunk cache center and radius.
- Flat map chunks around spawn.
- Chunk batch start and finish.
- Light data for spawn chunks.
- Default spawn position.
- Time update.
- Player abilities.
- Initial player position sync.
- Keepalive serverbound and clientbound in play.
- Basic movement packets accepted from client.

## Deferred

- Compression.
- Encryption.
- Online-mode session verification.
- Chat signing.
- Full vanilla registry coverage beyond evidence-driven first milestone
  entries.
- Persistent or mutable chunk packets.
- Resource pack negotiation.
- Complete play packet set.

## Current Vanilla Boundary

The server-list status path is vanilla-shaped for `1.21.11`.
The login path reaches configuration, negotiates the vanilla core pack,
loads the required non-empty registries, enters play, and sends a deterministic
`3x3` flat spawn chunk batch. Full terrain rendering by a stock client still
requires manual evidence because the registry and chunk set is intentionally
minimal.

Dynamic registries are intentionally minimal and evidence-driven. They are not
full vanilla coverage. `minecraft:damage_type` is required before play login can
construct the client level because vanilla play login expects
`minecraft:damage_type / minecraft:in_fire` to exist.

The target remains stock Minecraft Java Edition `1.21.11`. Fabric or modded
client reports may be recorded only when they expose a vanilla protocol gap.

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
12. Server sends play login, flat chunks, light, position, and keepalive.

## Rule

Every implemented packet must have encode or decode tests for boundary values
where practical.
