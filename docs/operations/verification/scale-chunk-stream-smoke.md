# Scale Chunk Stream Smoke

## Goal

Verify that a larger configured chunk radius streams progressively instead of
blocking play bootstrap on the full visible square.

## Setup

- Service: `scale-server`.
- Config: `config/verify/scale-server.json`.
- Probe: `cargo run -- probe scale-chunk-stream --host scale-server:25565`.
- Configured `view_distance`: `4`.

## Required Behavior

1. Login advertises view distance `4`.
2. `chunk_cache_radius` advertises radius `4`.
3. Initial bootstrap sends only `25` chunks.
4. Follow-up chunk batches contain at most `16` chunks.
5. The client eventually receives `81` unique chunks for the initial center.
6. Movement refreshes the target window and continues budgeted streaming.
7. Every streamed chunk validates as `level_chunk_with_light`.

## Rules

1. The existing `chunk-stream` probe remains the radius `2` regression gate.
2. This probe owns progressive larger-radius behavior.
3. Failure blocks scale-streaming acceptance.
