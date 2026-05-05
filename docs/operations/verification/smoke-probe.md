# Smoke Probe

## Goal

Verify the real wire path without depending on an external Minecraft bot crate.

## Probe Steps

1. Open TCP connection to `server:25565`.
2. Send status handshake and request.
3. Validate protocol `774` and version name `1.21.11`.
4. Send ping and validate matching pong.
5. Open a second TCP connection.
6. Send login handshake and offline login hello.
7. Acknowledge login success.
8. Validate login success has no trailing payload bytes.
9. Complete known-pack, registry, tag, and feature-flag configuration.
10. Enter play state and validate play login, the level-chunk readiness game
    event, the advertised chunk radius, level chunks, light, position,
    movement, time, and keepalive packets.

## Chunk Assertions

The smoke probe must validate enough live chunk payload data to catch the
captured client regressions:

- advertised radius `2` yields `25` chunks,
- each `level_chunk_with_light` payload has exactly consumed chunk data,
- each `update_light` payload has exactly consumed light data,
- heightmaps use `37` raw longs for each `9`-bit `256`-entry heightmap,
- section paletted containers omit VarInt raw-long lengths,
- biome containers use the plains single-value shape,
- the batch-finished size equals the observed chunk count.

## Keepalive Assertions

The smoke probe must answer the bootstrap keepalive and still observe the next
periodic keepalive. This proves the play loop can read client packets and keep
writing timed server packets during terrain loading.

## Play Loop Assertions

- The probe sends a vanilla-shaped `position_look` movement packet after
  teleport confirm.
- The probe must observe at least one periodic time packet before the next
  keepalive.

## Rules

- Probe code uses first-party packet framing.
- Probe failures print the phase name.
- Probe runs in Docker Compose.
- Probe assertions must use vanilla-shaped packet IDs and payloads, not
  probe-only play marker packets.
- Passing smoke proves the first-party login/configuration/play boundary, not
  final stock-client terrain rendering.
