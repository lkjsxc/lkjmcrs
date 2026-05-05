# Play Keepalive Timeout Report

## Source

Captured client log pasted on `2026-05-05 14:26:11 +0900`.

Client context:

- Minecraft Java Edition `1.21.11`
- Client-side mod warnings are present
- Evidence is accepted for the vanilla play-session timeout it exposes

## Packet Boundary

- Protocol phase: play
- Flow: clientbound silence after world entry
- Final disconnect text: `Timed out`

## Interpretation

The heightmap-size warnings are gone, so the client moved past the previous
`level_chunk_with_light` boundary. The remaining failure happens after roughly
`30` seconds in play state.

The current server sends one initial keepalive during bootstrap and then blocks
waiting for serverbound packets. A vanilla client expects the server to keep
sending play keepalives while the session is open.

## Required Follow-Up

Change the play loop so packet reads and periodic keepalive writes can progress
together. Send clientbound keepalives every `10` seconds and keep accepting
movement, teleport-confirm, chunk-batch, and keepalive responses.
