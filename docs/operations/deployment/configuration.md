# Configuration

## Canonical Config

Server configuration is a committed JSON file selected by:

```bash
lkjmcrs serve --config config/default.json
```

Environment variables are not a server configuration contract. Compose files
must select an explicit JSON config file.

## Fields

- `schema`: config schema identifier, currently `lkjmcrs.config`.
- `bind`: bind address, default `0.0.0.0:25565`.
- `motd`: status MOTD, default `lkjmcrs 1.21.11`.
- `max_players`: status max players, default `100`.
- `online_mode`: authentication mode, default `false`.
- `data_dir`: world and player storage root, default `data`.
- `default_game_mode`: new-profile game mode, default `creative`.
- `survival_starter_stone`: new-survival-profile starter stone count, default
  `0`.
- `view_distance`: advertised and streamed chunk radius, default `2`.
- `simulation_distance`: advertised simulation radius, default equals
  `view_distance`.
- `ops`: operator player names, default `[]`.

## Rules

1. Missing optional JSON fields use documented defaults.
2. Invalid numeric fields fail startup.
3. `online_mode: true` fails startup until online mode is implemented.
4. Secrets are not required for the first milestone.
5. `data_dir` must be writable by the server process before TCP bind.
6. `default_game_mode` accepts only `creative` or `survival`.
7. `survival_starter_stone` must be between `0` and `64`.
8. View and simulation distances must be between `2` and `8`.
9. `ops` names are matched case-insensitively by exact player name.
