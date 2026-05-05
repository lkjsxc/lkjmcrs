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

## Reach Boundary

- Session code validates block interaction reach before submitting mutations.
- The maximum accepted block reach is `6.0` blocks from player eye position to
  the target block center.
- Out-of-reach placement and breaking reconcile the target block through loaded
  chunk lookup only.
- Rejected reach checks must not load chunks, mutate chunks, or consume
  inventory.

## Region Ownership

- The initial region actor owns spawn chunks and chunks loaded by movement
  streaming in this slice.
- Sessions submit block mutations to the owning region actor.
- Region actors return the resulting block state for client reconciliation.
- Accepted mutations in loaded chunks are published as single-block updates to
  subscribed play sessions.
- Missing owners or still-unloaded chunks must not create chunks through the
  mutation path.
- Persistent override writes follow
  [persistent-overrides.md](persistent-overrides.md).

## Rules

1. No mutable world state is changed directly from session code.
2. Chunk snapshots are immutable values produced by region-owned state.
3. The flat generator remains deterministic and cheap to regenerate.
4. Persistence stores sparse overrides only and never replaces flat generation.
