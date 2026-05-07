# Protocol State Machine

## Status Flow

1. Client sends handshake with next state `status`.
2. Client sends status request.
3. Server sends JSON status response.
4. Client may send ping.
5. Server sends matching pong.
6. Connection may close.

## Login Flow

1. Client sends handshake with next state `login`.
2. Server validates protocol number.
3. Client sends login hello.
4. Offline mode creates a deterministic offline UUID.
5. Online mode performs encryption and session verification.
6. Server sends login success with the authoritative UUID.
7. Client sends login acknowledged.
8. Connection enters configuration.
9. Server sends known-pack selection.
10. Client selects `minecraft:core`.
11. Server sends minimal registry data and tags.
12. Server sends feature flags and finish configuration.
13. Client acknowledges finish configuration.
14. Connection enters play.

## Play Flow

- Server sends initial play state and spawn position.
- Server sends game event `13`, start waiting for level chunks.
- Server advertises the configured chunk-cache radius.
- Server sends a matching flat chunk and light batch around spawn.
- Server sends an initial keepalive, then sends another keepalive every `10`
  seconds while the play session remains open.
- Server sends observable time updates every `1` second after bootstrap.
- Client keepalive responses are accepted without blocking outgoing keepalives.
- Client movement updates session position.
