# Configuration

## Canonical Runtime

- `lkjmcrs serve` has no config-path argument.
- Built-in defaults are enough to start a local server.
- If `config/server.json` exists in the process working directory, the server
  reads it once during startup and overlays it on the built-in defaults.
- `config/server.json` is the only active runtime config file.
- Environment variables are not a server configuration contract.
- Example configs live in docs, not in `config/`.
- Config rewrites require process restart or container recreate.
- Config rewrites must not require Docker image rebuild.

## Fields

- `bind`: bind address, default `0.0.0.0:25565`.
- `motd`: status MOTD, default `lkjmcrs 1.21.11`.
- `max_players`: status max players, default `100`.
- `online_mode`: authentication mode, default `false`.
- `data_dir`: world and player storage root, default `data`.
- `default_game_mode`: new-profile game mode, default `survival`.
- `view_distance`: advertised and streamed chunk radius, default `2`.
- `simulation_distance`: advertised simulation radius, default equals
  `view_distance`.
- `ops`: operator player names, default `[]`.
- Offline-mode exposure policy requires shared configs to keep `ops: []`.

## Rules

1. Missing optional JSON fields use documented defaults.
2. Invalid numeric fields fail startup.
3. Unknown JSON fields fail startup.
4. `online_mode: true` fails startup until online mode is implemented.
5. `data_dir` must be writable by the server process before TCP bind.
6. `default_game_mode` accepts only `creative` or `survival`.
7. View and simulation distances must be between `2` and `8`.
8. `ops` names are matched case-insensitively by exact player name.
9. There is no starter-item config field.
10. Checked-in shared config must not grant operator names.

## Example

```json
{
  "bind": "0.0.0.0:25565",
  "motd": "lkjmcrs 1.21.11",
  "max_players": 100,
  "online_mode": false,
  "data_dir": "data",
  "default_game_mode": "survival",
  "view_distance": 2,
  "simulation_distance": 2,
  "ops": []
}
```
