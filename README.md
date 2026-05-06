# lkjmcrs

`lkjmcrs` is a Rust rewrite of the Minecraft Java Edition server, targeting
Minecraft `1.21.11`.

The project is docs-first. Start at [docs/README.md](docs/README.md), then
change implementation only after the relevant contract is clear.

## Current Milestone

- Rust-only server core.
- First-party minimal `1.21.11` protocol layer.
- Offline-mode playable skeleton first.
- Folia-inspired region ownership and asynchronous task handoff.
- Docker Compose verification as the required acceptance path.

## Verification

The canonical acceptance flow lives in
[docs/operations/verification/compose-pipeline.md](docs/operations/verification/compose-pipeline.md).
