# Flat World

## Goal

Provide deterministic terrain for login and smoke verification before full
world generation exists.

## Shape

- Bedrock layer at `y=0`.
- Stone from `y=1` through `y=62`.
- Dirt from `y=63` through `y=78`.
- Grass block at `y=79`.
- Air above `y=79`.
- Spawn at `0, 80, 0`.
- Default chunk-cache radius `2`, centered on spawn chunk `0,0`.
- Default initial terrain batch contains `25` chunks in a `5x5` square.

## Rules

1. Terrain generation is deterministic by chunk coordinate.
2. Generated chunks are immutable until block mutation exists.
3. Spawn chunk generation derives directly from the configured client radius.
4. The current slice may keep generated chunks in memory.
5. Sparse override persistence is layered over the deterministic base.
