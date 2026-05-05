# Heightmap Long Count Report

## Source

Captured client log pasted on `2026-05-05 14:18:40 +0900`.

Client context:

- Minecraft Java Edition `1.21.11`
- Client-side mod warnings are present
- Evidence is accepted for the vanilla chunk heightmap protocol gap it exposes

## Packet Boundary

- Packet: `clientbound/minecraft:level_chunk_with_light`
- Protocol phase: play
- Flow: clientbound
- Warning repeated for each spawn chunk and each sent heightmap:
  `Ignoring heightmap data for chunk [...], size does not match; expected: 37, got: 36`

## Interpretation

The previous paletted-container overrun is gone. The client enters chunk
handling, accepts the chunk section stream, and then rejects the heightmap long
array because the server emits compact bit-stream packing.

For `256` heightmap values at `9` bits per value, vanilla expects fixed
values-per-long packing with `7` values per `i64`, producing `37` longs.

## Required Follow-Up

Update heightmap encoding to use fixed values-per-long packing and add tests
that parse `level_chunk_with_light` heightmaps with `37` longs each.

## Resolution

Implemented fixed values-per-long heightmap storage and tests for two `37`-long
heightmaps per `level_chunk_with_light` packet.
