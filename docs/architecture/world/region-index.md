# Region Index

## Goal

Use compact, efficient world indexes that are easy for LLMs to reason about.

## Coordinate Rules

- Chunk coordinates are signed `i32` pairs.
- Region sections group chunks by power-of-two shifts.
- Packed keys use deterministic bit packing.
- Public helpers hide bit manipulation from higher-level modules.

## First Data Structures

- `ChunkPos`: typed chunk coordinate.
- `RegionSection`: typed region-section coordinate.
- `RegionId`: stable slot identifier.
- `RegionDirectory`: sparse section-to-region lookup plus region slots.

## Rules

1. No generic ECS is introduced in the current slice.
2. Use contiguous vectors where stable slot ownership is enough.
3. Use hash maps only for sparse coordinate lookup.
4. Tests cover negative coordinate packing.
