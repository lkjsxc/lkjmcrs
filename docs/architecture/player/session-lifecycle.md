# Session Lifecycle

## States

- `Handshake`: first packet decides status or login.
- `Status`: server-list response and ping.
- `Login`: version validation and profile creation.
- `Configuration`: registry and feature setup.
- `Play`: world entry, movement, keepalive, ticks.
- `Closed`: disconnected or failed.

## Session Data

- connection id,
- player name,
- UUID,
- loaded persistent player profile,
- current position,
- current yaw and pitch,
- on-ground and horizontal-collision flags,
- last keepalive id,
- last keepalive timestamp,
- current world age and day time,
- protocol state.

## Rules

1. Session owns packet I/O.
2. World mutation requests go through scheduler APIs.
3. Disconnect must save player state and release session-owned resources.
4. Session position updates are accepted only in play state.
5. Play-state packet reads must not prevent periodic keepalive or time writes.
