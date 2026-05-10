# lkjmcrs Documentation Canon

`docs/` is the only active canon for product behavior, architecture,
operations, and repository rules.

## System Goal

- Build a Rust Minecraft Java Edition server for `1.21.11`.
- Scale toward a region-owned server architecture inspired by PaperMC/Folia
  runtime lessons without targeting their plugin APIs.
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

1. [active-work.md](active-work.md)
2. [vision/current-status.md](vision/current-status.md)
3. [vision/purpose.md](vision/purpose.md)
4. [vision/principles.md](vision/principles.md)
5. [architecture/protocol/minecraft-1-21-11.md](architecture/protocol/minecraft-1-21-11.md)
6. [architecture/runtime/process-model.md](architecture/runtime/process-model.md)
7. [architecture/scheduler/region-ownership.md](architecture/scheduler/region-ownership.md)
8. [architecture/world/terrain-pipeline.md](architecture/world/terrain-pipeline.md)
9. [architecture/world/water-and-rivers.md](architecture/world/water-and-rivers.md)
10. [architecture/world/caves.md](architecture/world/caves.md)
11. [architecture/world/storage-schema.md](architecture/world/storage-schema.md)
12. [architecture/player/movement-authority.md](architecture/player/movement-authority.md)
13. [product/playable-server.md](product/playable-server.md)
14. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
15. [operations/verification/benchmark-plan.md](operations/verification/benchmark-plan.md)
16. [operations/verification/soak-plan.md](operations/verification/soak-plan.md)
17. [repository/workflow/change-sequence.md](repository/workflow/change-sequence.md)
