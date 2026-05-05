# Terrain Radius Timeout Report

## Source

Captured client log pasted on `2026-05-05 14:39:40 +0900`.

Client context:

- Minecraft Java Edition `1.21.11`
- Client-side mod warnings are present
- Evidence is accepted for the vanilla terrain-loading timeout it exposes

## Packet Boundary

- Protocol phase: play
- Flow: client reaches terrain loading after play bootstrap starts
- Final user-visible behavior: terrain loading waits, then times out

## Interpretation

The log no longer shows the earlier banner-pattern and mixin warnings as a
server blocker. The important user observation is that the client remains in
terrain loading long enough to time out.

The server advertises chunk-cache radius `2`, which describes a `5x5` square
around the center chunk. The implementation sent only a `3x3` square. That
mismatch can leave the client waiting for terrain it has been told is in the
initial cache.

## Required Follow-Up

Derive the bootstrap chunk batch from the advertised radius:

- radius `2` sends `25` chunks,
- `chunk_batch_finished` reports the derived chunk count,
- the first-party probe validates the same derived count.

## Resolution

Implemented in the server bootstrap and first-party probe. The next evidence
step is a fresh stock-client join attempt against the fixed full-radius batch.
