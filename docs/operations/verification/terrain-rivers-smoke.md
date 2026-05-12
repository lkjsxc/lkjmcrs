# Terrain Rivers Smoke

## Goal

Verify that natural terrain sends static water and river blocks through the
public chunk stream.

## Required Scenario

1. Start `river-terrain-server` through Docker Compose.
2. Join with a first-party probe client.
3. Complete login, configuration, and play bootstrap.
4. Decode initial `level_chunk_with_light` packets.
5. Require at least one `minecraft:water[level=0]` block-state ID near spawn.
6. Require decoded water top at the documented river and ocean level.
7. Require at least one non-flat generated surface near spawn.

## Assertions

- Water appears through the same chunk packet path as generated terrain.
- Water is not stranded far below ordinary nearby terrain.
- River terrain does not require persisted overrides.
- Embedded light remains valid.
- Chunk batch count still matches the advertised radius.

## Gate Command

- Compose service: `river-terrain`.
