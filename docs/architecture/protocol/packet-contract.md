# Packet Contract

## Login

Target protocol: `774` for Minecraft Java Edition `1.21.11`.

- `login/clientbound 0x02 success`: UUID, username string, property array.
- The property array is VarInt-counted entries of name, value, optional
  signature.
- There is no trailing boolean after the property array.
- `login/serverbound 0x03 login_acknowledged` has an empty payload.

## Configuration

- `0x0e select_known_packs`: sends `minecraft:core` version `1.21.11`.
- `0x07 registry_data`: one packet per declared dynamic registry.
- `0x0d tags`: tag groups for declared dynamic registries.
- `0x0c feature_flags`: sends `minecraft:vanilla`.
- `0x03 finish_configuration`: empty payload.

The first milestone declares one registry packet for each of these registries:

- `minecraft:dimension_type` with `minecraft:overworld` at registry ID `0`.
- `minecraft:worldgen/biome` with `minecraft:plains` at registry ID `0`.
- `minecraft:damage_type` with the bootstrap keys in
  [dynamic-registries.md](dynamic-registries.md).
- Minimal non-empty variant registries required by the vanilla client:
  cat, chicken, cow, frog, painting, pig, wolf sound, wolf, and zombie nautilus.
- `minecraft:timeline` with `minecraft:day` at registry ID `0`.
- `minecraft:timeline` tag `minecraft:in_overworld` binds to ID `0`.

## Play Bootstrap

- `0x30 login`: one world, `minecraft:overworld`; view and simulation
  distance are `2`.
- `0x5c update_view_position`: spawn chunk `0,0`.
- `0x5d set_chunk_cache_radius`: radius `2`.
- `0x0c chunk_batch_start`: empty payload.
- `0x2c level_chunk_with_light`: flat chunk data and light arrays.
- `0x2f update_light`: explicit light data for the same chunk, retained for the
  current join milestone.
- `0x0b chunk_batch_finished`: batch size.
- `0x5f spawn_position`: global position in `minecraft:overworld`.
- `0x6f update_time`: age `0`, time `0`, ticking enabled.
- `0x3e abilities`: permissive initial ability flags.
- `0x46 position`: spawn teleport with teleport ID `1`.
- `0x2b keep_alive`: signed 64-bit keepalive ID.

## Flat Chunk IDs

Default block-state IDs are pinned from `minecraft-data` `1.21.11`:

- air: `0`
- stone: `1`
- grass block default: `9`
- dirt: `10`
- bedrock: `85`
