# Lifecycle

## Startup

1. Load configuration from JSON.
2. Reject unsupported options.
3. Initialize player storage, world storage, and scheduler state.
4. Bind the TCP listener only after storage opens successfully.
5. Accept client connections.

## Connection Lifecycle

1. Read handshake.
2. Route to status or login.
3. Validate protocol number for login.
4. Complete offline identity or online encrypted identity.
5. Load or create the UUID-owned player profile.
6. Send configuration data.
7. Enter play state.
8. Run keepalive, time, vitals, and chunk-visible updates.

## Shutdown

- Stop accepting new connections.
- Disconnect active sessions.
- Stop scheduler workers.
- Accepted mutations update memory before disk flush; storage catches up through
  the documented region-owned persistence path.
