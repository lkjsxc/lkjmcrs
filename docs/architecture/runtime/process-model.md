# Process Model

## Runtime

- Use Tokio for async networking and timers.
- One process owns protocol, sessions, world state, and schedulers.
- Tick work is isolated from blocking filesystem or network I/O.
- Configuration is read once at startup for the first milestone.

## Services

- TCP listener accepts Minecraft connections.
- Connection tasks own packet I/O for one client.
- Session state is separate from world region state.
- Region scheduler owns world mutation.
- Probe commands reuse protocol code where possible.

## Failure Policy

- Invalid packet frames disconnect the connection.
- Unsupported protocol versions receive a clear disconnect.
- Unsupported `online_mode=true` aborts startup.
- Internal task failures are logged and trigger graceful disconnect when tied to a player.
