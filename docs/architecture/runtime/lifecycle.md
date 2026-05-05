# Lifecycle

## Startup

1. Load configuration from environment.
2. Reject unsupported options.
3. Initialize world and scheduler state.
4. Bind the TCP listener.
5. Accept client connections.

## Connection Lifecycle

1. Read handshake.
2. Route to status or login.
3. Validate protocol version for login.
4. Complete offline login.
5. Send configuration data.
6. Enter play state.
7. Run keepalive and tick-visible updates.

## Shutdown

- Stop accepting new connections.
- Disconnect active sessions.
- Stop scheduler workers.
- Flush accepted persistent world overrides before acknowledging mutations.
