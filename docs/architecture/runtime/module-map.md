# Module Map

## Top-Level Modules

- `app`: CLI dispatch and command execution.
- `config`: JSON configuration and defaults.
- `net`: TCP listener and connection loop.
- `protocol`: packet framing, types, encoding, decoding, and wire DTOs.
- `player`: persistent player model, inventory, named locations, and `redb`
  storage.
- `session`: login, configuration, play state, keepalive, commands, and travel
  command dispatch.
- `world`: chunk, region, terrain, and `WorldStore` storage data.
- `scheduler`: region actor and task ownership primitives.
- `quality`: docs topology and line-limit checks.
- `probe`: status and login/play smoke probes over the public wire path.

## Dependency Rules

1. `world` does not depend on network modules.
2. `protocol` does not depend on scheduler or world modules.
3. `player` does not depend on protocol, scheduler, session, or world modules.
4. `session` coordinates protocol, player state, and world access.
5. `scheduler` owns mutation entrypoints for region state.
6. `probe` may depend on protocol and documented acceptance constants, but it
   must not call server runtime paths instead of the public wire.

## Notable Submodules

- `player::store_json`: profile and inventory JSON mapping.
- `player::location_json`: home and warp JSON mapping.
- `world::storage_codec`: binary sparse override value mapping.
- `session::command_dispatch`: command permission and routing entrypoint.
- `session::travel_commands`: `/spawn`, homes, and warps.

The strict protocol boundary is owned by
[../protocol/module-boundary.md](../protocol/module-boundary.md).
