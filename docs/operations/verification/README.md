# Verification

Use this subtree for required compose verification contracts.

## Read This Section When

- You need the canonical acceptance command sequence.
- You need smoke probe scope.
- You need failure policy.

## Pipeline And Results

- [compose-pipeline.md](compose-pipeline.md): required compose commands.
- [current-results.md](current-results.md): latest compose verification result.
- [results/README.md](results/README.md): historical compose summaries.
- [evidence-policy.md](evidence-policy.md): how reports become active evidence.

## Protocol And Join

- [smoke-probe.md](smoke-probe.md): wire smoke behavior.
- [online-auth-smoke.md](online-auth-smoke.md): encrypted online login.
- [online-vanilla-join.md](online-vanilla-join.md): manual online-mode client
  evidence.
- [manual-client-boundary.md](manual-client-boundary.md): active manual client
  boundary.
- [join-boundary.md](join-boundary.md): latest known manual join boundary.
- [vanilla-join.md](vanilla-join.md): manual stock-client join checks.
- [client-reports/README.md](client-reports/README.md): captured client
  disconnect evidence.

## World And Movement

- [terrain-generation-smoke.md](terrain-generation-smoke.md): natural terrain
  and spawn smoke.
- [terrain-rivers-smoke.md](terrain-rivers-smoke.md): static water and river
  chunk smoke.
- [terrain-caves-smoke.md](terrain-caves-smoke.md): generated underground cave
  chunk smoke.
- [worldgen-golden.md](worldgen-golden.md): deterministic terrain golden target.
- [chunk-border-property.md](chunk-border-property.md): generated border
  property target.
- [render-distance-smoke.md](render-distance-smoke.md): radius `32` terrain
  streaming smoke.
- [storage-section-persistence.md](storage-section-persistence.md): binary
  override persistence target.
- [movement-authority-smoke.md](movement-authority-smoke.md): movement trust
  boundary smoke target.

## Gameplay And Scale

- [block-mutation-smoke.md](block-mutation-smoke.md): live block mutation.
- [multiplayer-mutation-smoke.md](multiplayer-mutation-smoke.md): observer
  fanout.
- [profile-reconnect-smoke.md](profile-reconnect-smoke.md): player profile
  persistence.
- [chunk-stream-smoke.md](chunk-stream-smoke.md): movement-driven streaming.
- [scale-chunk-stream-smoke.md](scale-chunk-stream-smoke.md): larger-radius
  streaming.
- [load-and-metrics.md](load-and-metrics.md): scale metrics.
- [smp-commands-smoke.md](smp-commands-smoke.md): chat and commands.
- [survival-item-smoke.md](survival-item-smoke.md): survival item loop.
- [inventory-sync-smoke.md](inventory-sync-smoke.md): inventory projection.
- [item-pickup-smoke.md](item-pickup-smoke.md): dropped item pickup.
- [survival-vitals-smoke.md](survival-vitals-smoke.md): damage and respawn.
- [persistence-smoke.md](persistence-smoke.md): persisted block overrides.
- [benchmark-plan.md](benchmark-plan.md): benchmark lanes.
- [soak-plan.md](soak-plan.md): long-running lanes.
