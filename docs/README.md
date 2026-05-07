# lkjmcrs Documentation Canon

`docs/` is the only active canon for product behavior, architecture,
operations, and repository rules.

## System Goal

- Build a Rust Minecraft Java Edition server for `1.21.11`.
- Scale toward a PaperMC/Folia-class server architecture.
- Keep gameplay mutation safe through region ownership and asynchronous handoff.
- Support large redstone and entity contraptions without a single global tick
  thread becoming the permanent bottleneck.
- Add original `lkjmcsmp`-style gameplay after the basic server is working.

## Global Rules

1. Docs are authoritative; implementation follows docs.
2. Update docs before behavior-changing code.
3. Keep one canonical owner for each contract.
4. Keep every docs directory to one `README.md` plus multiple children.
5. Keep docs files at `<= 300` lines.
6. Keep authored source files at `<= 200` lines.
7. Prefer exact defaults, protocol constants, commands, and file paths.
8. Remove conflicting old contracts instead of preserving compatibility.
9. Verify through Docker Compose before accepting implementation batches.

## Top-Level Sections

- [vision/README.md](vision/README.md): project purpose, principles, and
  capability order.
- [getting-started/README.md](getting-started/README.md): orientation and first run.
- [product/README.md](product/README.md): user-visible behavior and compatibility.
- [architecture/README.md](architecture/README.md): protocol, runtime, scheduler, world, player.
- [operations/README.md](operations/README.md): verification, deployment, quality.
- [repository/README.md](repository/README.md): layout, workflow, and repository rules.

## Recommended Reading Order

1. [vision/current-status.md](vision/current-status.md)
2. [vision/purpose.md](vision/purpose.md)
3. [vision/principles.md](vision/principles.md)
4. [architecture/protocol/minecraft-1-21-11.md](architecture/protocol/minecraft-1-21-11.md)
5. [architecture/runtime/process-model.md](architecture/runtime/process-model.md)
6. [architecture/scheduler/region-ownership.md](architecture/scheduler/region-ownership.md)
7. [architecture/world/region-index.md](architecture/world/region-index.md)
8. [product/playable-server.md](product/playable-server.md)
9. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
10. [repository/workflow/change-sequence.md](repository/workflow/change-sequence.md)
