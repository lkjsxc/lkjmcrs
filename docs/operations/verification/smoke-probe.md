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
10. Enter play state and observe play login, the level-chunk readiness game
    event, the full advertised chunk radius, light, position, and keepalive
    packets.

## Rules

- Probe code uses first-party packet framing.
- Probe failures print the phase name.
- Probe runs in Docker Compose.
- Probe assertions must use vanilla-shaped packet IDs and payloads, not
  probe-only play marker packets.
- Passing smoke proves the first-party login/configuration/play boundary, not
  final stock-client terrain rendering.
