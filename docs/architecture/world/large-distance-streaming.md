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
- The accepted `view_distance` range is `2..=32`.
- The accepted `simulation_distance` range remains `2..=8`.
- Radius `32` means eventual convergence to `4225` chunks, not eager
  full-square login.

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

## Radius 32 Budgets

- Drain interval: `50ms`.
- Maximum chunks per follow-up batch: `16`.
- Maximum level-chunk payload bytes per follow-up batch: `1048576`.
- Pending drains issue one region `load_chunks` request for the selected batch.
- Radius `32` requires `25` initial chunks and `4225` eventual unique chunks.
- Flat radius `4` and `8` verification remain regression evidence.

## Cache Boundary

- Generated flat chunks may reuse a cached payload body.
- Chunk coordinates stay outside the cached body.
- Chunks with sparse overrides bypass the flat payload cache.
- Unmodified natural chunks may use a bounded process-shared payload cache keyed
  by terrain kind, world seed, and chunk position.
- The natural cache cap is `8192` full chunk payloads with FIFO eviction.
- Natural chunks with sparse overrides bypass the shared generated payload cache.

## Rules

1. Radius `2` behavior stays wire-identical unless this doc changes first.
2. Do not raise the configured cap above `32` until progressive streaming is
   compose-verified with load evidence for the later scale gate.
3. Do not full-square bootstrap for larger radii.
4. Packet compression is future work and is not part of this acceptance batch.
5. Radius `128` is not exposed or verified in this batch.
