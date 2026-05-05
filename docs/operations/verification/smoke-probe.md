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
7. Complete configuration acknowledgement.
8. Enter play state and observe keepalive or tick-visible packet.

## Rules

- Probe code uses first-party packet framing.
- Probe failures print the phase name.
- Probe runs in Docker Compose.
