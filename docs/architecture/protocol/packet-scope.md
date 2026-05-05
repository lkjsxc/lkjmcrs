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
- Keepalive serverbound and clientbound.
- Basic movement packets accepted from client.
- First-party play-ready probe packet.

## Deferred

- Compression.
- Encryption.
- Online-mode session verification.
- Chat signing.
- Full registry fidelity.
- Vanilla-complete chunk packets.
- Resource pack negotiation.
- Complete play packet set.

## Rule

Every implemented packet must have encode or decode tests for boundary values
where practical.
