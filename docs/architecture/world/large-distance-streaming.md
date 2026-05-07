# Large Distance Streaming

## Goal

Make larger configured chunk radii practical without blocking play bootstrap on
a full square of chunks.

## Radius Model

- `view_distance` remains the advertised client cache radius.
- `simulation_distance` remains an advertised ticking radius and does not drive
  chunk streaming in this slice.
- Radius `2` is the near bootstrap radius.
- Configured radii above `2` are streamed progressively after play begins.
- The current accepted configured range remains `2..=8`.

## Delivery Contract

1. The login packet advertises the configured `view_distance`.
2. The `chunk_cache_radius` packet advertises the same configured radius.
3. Initial bootstrap sends only the near radius when `view_distance` is above
   `2`.
4. Farther chunks are queued by Chebyshev ring from the current center.
5. Queued chunks are sent in small follow-up chunk batches.
6. Each follow-up batch has explicit chunk and byte budgets.
7. At least one queued chunk may be sent even if its payload exceeds the byte
   budget.
8. Sessions subscribe to a chunk only after the chunk is sent.
9. Chunks not yet sent are not unloaded.
10. Movement replaces stale queued chunks with the new target window.

## First Budgets

- Drain interval: `100ms`.
- Maximum chunks per follow-up batch: `8`.
- Maximum level-chunk payload bytes per follow-up batch: `524288`.

## Cache Boundary

- Generated flat chunks may reuse a cached payload body.
- Chunk coordinates stay outside the cached body.
- Chunks with sparse overrides bypass the flat payload cache.
- Terrain-dependent cache keys must be redesigned before non-flat terrain uses
  the same cache.

## Rules

1. Radius `2` behavior stays wire-identical unless this doc changes first.
2. Do not raise the configured cap until progressive streaming is
   compose-verified.
3. Do not full-square bootstrap for larger radii.
