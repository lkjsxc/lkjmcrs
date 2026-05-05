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
7. Server sends minimal configuration data.
8. Server sends finish configuration.
9. Client acknowledges finish configuration.
10. Connection enters play.

## Play Flow

- Server sends initial play state and spawn position.
- Server sends flat chunks around spawn.
- Server sends periodic keepalive.
- Client movement updates session position.
