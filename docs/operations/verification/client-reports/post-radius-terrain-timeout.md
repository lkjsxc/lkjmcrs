# Post-Radius Terrain Timeout Report

## Source

Captured client log pasted on `2026-05-05 14:58:38 +0900`.

Client context:

- Minecraft Java Edition `1.21.11`
- Client-side mod warnings are present
- Evidence is accepted for the remaining terrain-loading timeout

## Packet Boundary

- Protocol phase: play
- Flow: client connects, starts render workers, reaches terrain loading, then
  times out
- Previous radius mismatch fix is not sufficient

## Interpretation

The latest log still has no client-side packet decoding exception. That means
the client is likely waiting for an expected play-state readiness signal after
the initial world data, rather than rejecting an already decoded payload.

The next implementation pass should focus on the post-chunk bootstrap sequence:

- whether the client receives an explicit start-waiting-for-chunks game event,
- whether player position is sent before or after enough chunk data,
- whether the server waits for and observes `player_loaded`,
- whether keepalives continue while the client stays in terrain loading.

## Required Follow-Up

Add the smallest vanilla-shaped play readiness packets that move the client out
of terrain loading, and extend the first-party probe so it proves their order.

## Resolution

Implemented game event `13`, start waiting for level chunks, before the
advertised chunk batch. The first-party probe validates the packet and payload.
The next evidence step is a fresh stock-client join attempt against this fixed
readiness sequence.
