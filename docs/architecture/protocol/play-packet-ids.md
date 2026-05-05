# Play Packet IDs

## Source Baseline

Protocol `774` is the only supported play protocol. Packet IDs are pinned from
the `1.21.11` protocol table and must be updated as one batch with
[version-1-21-11.md](version-1-21-11.md) during any version bump.

## Clientbound IDs Used By The First Milestone

- `0x0b chunk_batch_finished`
- `0x0c chunk_batch_start`
- `0x26 game_state_change`
- `0x2b keep_alive`
- `0x2c map_chunk`, locally named `level_chunk_with_light`
- `0x2f update_light`
- `0x30 login`
- `0x3e abilities`
- `0x46 position`
- `0x5c update_view_position`
- `0x5d update_view_distance`
- `0x5f spawn_position`
- `0x6f update_time`

## Serverbound IDs Used By The First Milestone

- `0x00 teleport_confirm`
- `0x0a chunk_batch_received`
- `0x0d settings`
- `0x1b keep_alive`
- `0x1d position`
- `0x1e position_look`
- `0x1f look`
- `0x20 flying`
- `0x2b player_loaded`
- `0x2c pong`

## Regression Rule

Never infer a packet ID from old names or neighboring Minecraft versions.
`0x21` is a chat packet in `1.21.11`, not a game event. Sending the level-chunk
readiness payload on `0x21` makes the vanilla client parse byte `13` as NBT and
disconnect before terrain can render.
