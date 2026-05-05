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
- current position,
- last keepalive id,
- last keepalive timestamp,
- protocol state.

## Rules

1. Session owns packet I/O.
2. World mutation requests go through scheduler APIs.
3. Disconnect must release session-owned resources.
4. Session position updates are accepted only in play state.
