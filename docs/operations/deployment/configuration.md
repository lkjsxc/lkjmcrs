# Configuration

## Environment Variables

- `LKJMCRS_BIND`: bind address, default `0.0.0.0:25565`.
- `LKJMCRS_MOTD`: status MOTD, default `lkjmcrs 1.21.11`.
- `LKJMCRS_MAX_PLAYERS`: status max players, default `100`.
- `LKJMCRS_ONLINE_MODE`: authentication mode, default `false`.
- `LKJMCRS_DATA_DIR`: world override storage root, default `data`.
- `LKJMCRS_DEFAULT_GAME_MODE`: new-profile game mode, default `creative`.
- `LKJMCRS_SURVIVAL_STARTER_STONE`: new-survival-profile starter stone count,
  default `0`.
- `LKJMCRS_VIEW_DISTANCE`: advertised and streamed chunk radius, default `2`.
- `LKJMCRS_SIMULATION_DISTANCE`: advertised simulation radius, default equals
  view distance.

## Rules

1. Missing variables use documented defaults.
2. Invalid numeric variables fail startup.
3. `LKJMCRS_ONLINE_MODE=true` fails startup until online mode is implemented.
4. Secrets are not required for the first milestone.
5. `LKJMCRS_DATA_DIR` must be writable by the server process.
6. `LKJMCRS_DEFAULT_GAME_MODE` accepts only `creative` or `survival`.
7. `LKJMCRS_SURVIVAL_STARTER_STONE` must be between `0` and `64`.
8. View and simulation distances must be between `2` and `8`.
