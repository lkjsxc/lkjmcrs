# Packet Scope

## Implemented First

- Handshake to status.
- Handshake to login.
- Status request and JSON response.
- Ping request and pong response.
- Login hello from client.
- Login success from server.
- Login acknowledged from client.
- Minimal configuration packets.
- Finish configuration from server.
- Finish configuration from client.
- Play login/join packet.
- Player position sync.
- Minimal chunk data for a flat spawn area.
- Keepalive serverbound and clientbound.
- Basic movement packets accepted from client.

## Deferred

- Compression.
- Encryption.
- Online-mode session verification.
- Chat signing.
- Full registry fidelity.
- Resource pack negotiation.
- Complete play packet set.

## Rule

Every implemented packet must have encode or decode tests for boundary values
where practical.
