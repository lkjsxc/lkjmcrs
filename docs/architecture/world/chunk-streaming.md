# Chunk Streaming

## Goal

Load and send newly visible flat chunks as a player crosses chunk-center
boundaries, without unloading client chunks in this slice.

## Coordinates

- The session chunk center is derived from player `x` and `z` by flooring each
  coordinate to a block coordinate and applying Euclidean division by `16`.
- Negative coordinates use the same mapping as block mutation lookup.
- The advertised view radius remains `2`, so each center has a `5x5` visible
  set.

## Movement Contract

1. A valid serverbound movement packet updates session-local position.
2. If the derived chunk center is unchanged, no streaming packets are sent.
3. If the center changes, the session computes the new radius-`2` visible set.
4. Chunks not already sent to that session are loaded through the region actor
   with `spawn_chunks_around`.
5. The server sends `chunk_cache_center` for the new center.
6. If there are newly sent chunks, the server sends a chunk batch containing
   only those chunks.
7. The session subscribes to newly sent chunks for future block-update fanout.

## Load-Only Boundary

This milestone never sends client unload packets. Chunks already sent to a
session remain in that session's subscription set until disconnect. Dynamic
region split and merge, entity streaming, survival inventory rules, and SMP
commands are out of scope.

## Mutation Boundary

Newly streamed chunks are normal loaded chunks. Creative-style placement and
breaking go through the region actor, sparse override persistence, prediction
acknowledgements, and observer fanout used by spawn chunks.
