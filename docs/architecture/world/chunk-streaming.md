# Chunk Streaming

## Goal

Keep each session's client chunk window bounded as the player crosses
chunk-center boundaries.

## Coordinates

- The session chunk center is derived from player `x` and `z` by flooring each
  coordinate to a block coordinate and applying Euclidean division by `16`.
- Negative coordinates use the same mapping as block mutation lookup.
- The configured view distance defaults to `2`, so the default center has a
  `5x5` visible set.
- The configured simulation distance is advertised during login but does not
  drive streaming in this slice.

## Movement Contract

1. A valid serverbound movement packet updates session-local position.
2. If the derived chunk center is unchanged, no streaming packets are sent.
3. If the center changes, the session computes the next visible set from the
   configured view distance.
4. Chunks leaving the previous visible set are sent as `unload_chunk`.
5. Chunks entering the next visible set are loaded through the region actor by
   exact chunk position.
6. The server sends `chunk_cache_center` for the new center.
7. If there are newly visible chunks, the server sends a chunk batch containing
   only those chunks.
8. The session registry unsubscribes leaving chunks and subscribes entering
   chunks for future block-update fanout.

## Bounded Window Boundary

`ChunkStream` tracks the current visible set, not all chunks ever sent. Moving
from center `0,0` to `1,0` with view distance `2` unloads column `x=-2` and
loads column `x=3`. Dynamic region split and merge, entity streaming, and using
simulation distance for ticking are out of scope.

## Distance Budget Boundary

The current configured view-distance range stays `2..=8`. Larger targets require
progressive loading before the cap changes:

- near chunks stream first,
- far chunks use explicit chunk and byte budgets,
- reusable encoded chunk payloads are cached for unchanged generated chunks,
- full square bootstrap is not used for large-distance targets.

## Mutation Boundary

Newly streamed chunks are normal loaded chunks. Creative-style placement and
breaking go through the region actor, sparse override persistence, prediction
acknowledgements, and observer fanout used by spawn chunks.
