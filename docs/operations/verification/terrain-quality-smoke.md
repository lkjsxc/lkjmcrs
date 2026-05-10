# Terrain Quality Smoke

## Goal

Verify that natural terrain sends survival-useful surface features through the
public chunk stream.

## Required Scenario

1. Start a natural-terrain server through Docker Compose.
2. Join with a first-party probe client.
3. Complete login, configuration, and play bootstrap.
4. Decode initial and nearby streamed `level_chunk_with_light` packets.
5. Require dry spawn footing with two-block headroom.
6. Require nearby static water access.
7. Require nearby tree or wood-bearing decorator output after decorators land.
8. Require non-flat terrain outside the near spawn area.

## Assertions

- Surface features appear through generated terrain, not persisted overrides.
- Spawn is resolved from the same generator state used for chunk output.
- Chunk batch count still matches the advertised radius.
- Embedded light remains valid for richer terrain sections.

## Gate Command

- Compose service: `terrain-quality` after the probe exists.
- Until then, `terrain-generation`, `river-terrain`, and `render-distance`
  remain the live coverage for this target.
