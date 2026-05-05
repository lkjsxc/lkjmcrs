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
2. Server validates protocol version.
3. Client sends login hello.
4. Server sends login success for offline mode.
5. Client sends login acknowledged.
6. Connection enters configuration.
7. Server sends known-pack selection.
8. Client selects `minecraft:core`.
9. Server sends minimal registry data and tags.
10. Server sends feature flags and finish configuration.
11. Client acknowledges finish configuration.
12. Connection enters play.

## Play Flow

- Server sends initial play state and spawn position.
- Server sends game event `13`, start waiting for level chunks.
- Server advertises chunk-cache radius `2`.
- Server sends a matching `5x5` flat chunk and light batch around spawn.
- Server sends an initial keepalive, then sends another keepalive every `10`
  seconds while the play session remains open.
- Server sends observable time updates every `1` second after bootstrap.
- Client keepalive responses are accepted without blocking outgoing keepalives.
- Client movement updates session position.
