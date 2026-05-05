# Module Map

## Top-Level Modules

- `app`: CLI dispatch and command execution.
- `config`: environment and default configuration.
- `net`: TCP listener and connection loop.
- `protocol`: packet framing, types, encoding, decoding.
- `session`: login, configuration, play state, keepalive.
- `world`: chunk, region, and flat-world data.
- `scheduler`: region actor and task ownership primitives.
- `quality`: docs topology and line-limit checks.
- `probe`: status and login/play smoke probes.

## Dependency Rules

1. `world` does not depend on network modules.
2. `protocol` does not depend on scheduler or world modules.
3. `session` coordinates protocol and world access.
4. `scheduler` owns mutation entrypoints for region state.
5. `probe` may depend on protocol but not on server internals.
