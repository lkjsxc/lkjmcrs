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

## Rules

1. Terrain generation is deterministic by chunk coordinate.
2. Generated chunks are immutable until block mutation exists.
3. First milestone may keep generated chunks in memory.
4. Persistence is not required until survival-core work begins.
