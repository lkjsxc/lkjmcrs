# Mutable Chunks

## Goal

Layer sparse block overrides on top of deterministic flat chunks so early block
interaction can be implemented without introducing persistence.

## Coordinates

- `BlockPos` stores signed `i32` `x`, `y`, and `z` coordinates.
- Chunk lookup uses Euclidean division by `16`, so negative block coordinates
  map to the expected negative chunks.
- Local block coordinates are stored as `0..15` for `x` and `z`.
- Writable block `y` coordinates are bounded by the world height used by chunk
  encoding: `-64..=319`.

## State Model

- A generated flat chunk is the immutable base.
- A mutable chunk stores only block overrides that differ from the base.
- Setting a block back to its generated base value removes the override.
- Section serialization reads through the same block lookup path as gameplay.
- Bedrock at `y=0` is immutable in the first mutation slice.

## Region Ownership

- The static spawn region owns all chunks in the advertised radius `2`.
- Sessions submit block mutations to the owning region actor.
- Region actors return the resulting block state for client reconciliation.
- Missing owners or unloaded chunks must not create chunks during this slice.

## Rules

1. No mutable world state is changed directly from session code.
2. Chunk snapshots are immutable values produced by region-owned state.
3. The flat generator remains deterministic and cheap to regenerate.
4. Persistence is out of scope until this contract is explicitly extended.
