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
- Online-mode encryption request and response for authenticated login.
- Basic movement packets decoded from client and stored on the session.
- Movement-driven `chunk_cache_center` updates, unload packets for chunks
  leaving the view window, and chunk batches for newly visible chunks.
- Stored player position, look, and game mode used during play bootstrap.
- Health, hunger, and saturation sent during play bootstrap.
- Creative and first survival item-loop placement and breaking in loaded
  chunks.
- Block prediction acknowledgements and single-block updates.
- Single-block updates fanned out to subscribed play sessions.
- Unsigned offline-mode chat and first slash commands.
- Operator-driven damage, death state, and respawn request handling.

## Deferred

- Compression.
- Chat signing.
- Full vanilla registry coverage beyond evidence-driven current entries.
- Full mutable chunk resend packets.
- Full client inventory windows and item synchronization.
- Resource pack negotiation.
- Complete play packet set.

## Current Vanilla Boundary

The server-list status path is vanilla-shaped for `1.21.11`.
Offline login and online encrypted login both reach configuration, negotiate the
vanilla core pack, load the required non-empty registries, enter play, send game
event `13`, and send deterministic spawn-near chunks for the advertised view
distance. Online login verifies the session profile before login success.
A player join was reported on `2026-05-05` after the `0x26` fix. Treat that as
manual success evidence without raw logs attached.

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
3. Online mode sends encryption request, validates encryption response, enables
   AES/CFB8, and verifies the session profile.
4. Server sends login success.
5. Client sends login acknowledged.
6. Client may send configuration settings.
7. Server sends known packs with `minecraft:core` release `1.21.11`.
8. Client replies with selected known packs.
9. Server sends registry data and tags.
10. Server sends enabled features with `minecraft:vanilla`.
11. Server sends finish configuration.
12. Client acknowledges finish configuration.
13. Server sends play login, game event `13`, the initial chunk batch, light,
    position, and keepalive. Radius behavior is owned by
    [../world/large-distance-streaming.md](../world/large-distance-streaming.md).

## Rule

Every implemented packet must have encode or decode tests for boundary values
where practical.
