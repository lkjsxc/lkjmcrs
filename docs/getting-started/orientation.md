# Orientation

## Project Shape

`lkjmcrs` starts as a small Rust server and grows through documented
contracts. The repo is intentionally new and has no compatibility burden.

## Start Paths

1. Read [../vision/purpose.md](../vision/purpose.md).
2. Read [../architecture/protocol/minecraft-1-21-11.md](../architecture/protocol/minecraft-1-21-11.md).
3. Read [../architecture/scheduler/region-ownership.md](../architecture/scheduler/region-ownership.md).
4. Run the commands in [verification.md](verification.md).

## Current Constraints

- Host Rust is not required.
- Docker Compose is required.
- Offline-mode and online-mode login paths are implemented.
- Public exposure requires `online_mode: true` and authenticated UUID identity.
- Public plugin API is intentionally deferred.
