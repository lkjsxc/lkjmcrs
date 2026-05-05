# Configuration

## Environment Variables

- `LKJMCRS_BIND`: bind address, default `0.0.0.0:25565`.
- `LKJMCRS_MOTD`: status MOTD, default `lkjmcrs 1.21.11`.
- `LKJMCRS_MAX_PLAYERS`: status max players, default `100`.
- `LKJMCRS_ONLINE_MODE`: authentication mode, default `false`.

## Rules

1. Missing variables use documented defaults.
2. Invalid numeric variables fail startup.
3. `LKJMCRS_ONLINE_MODE=true` fails startup until online mode is implemented.
4. Secrets are not required for the first milestone.
