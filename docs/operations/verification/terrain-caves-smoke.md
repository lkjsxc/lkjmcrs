# Terrain Caves Smoke

## Goal

Verify that natural terrain sends enclosed underground cave air through the
public chunk stream.

## Required Scenario

1. Start `cave-terrain-server` through Docker Compose.
2. Join with a first-party probe client.
3. Complete login, configuration, and play bootstrap.
4. Decode initial `level_chunk_with_light` packets.
5. Require at least one enclosed underground `minecraft:air` pocket below the
   decoded surface.
6. Require no cave opening through the decoded surface column.

## Assertions

- Cave air appears through the same chunk packet path as generated terrain.
- Cave terrain does not require persisted overrides.
- Static water columns remain intact.
- Chunk batch count still matches the advertised radius.

## Gate Command

- Compose service: `terrain-caves`.
