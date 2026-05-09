# Benchmark Plan

## Goal

Define the next benchmark lanes before adding broader scale features.

## Initial Targets

- World storage load and save latency for sparse override chunks.
- Chunk generation latency for flat, plateau, blended, and outer natural
  terrain.
- Encoded chunk payload cache hit rate for generated chunks.
- Region actor command latency under mixed movement and mutation load.
- Follow-up chunk streaming batch size and payload bytes at configured radii.

## Rules

1. Benchmarks must run outside session packet I/O hot paths.
2. Each benchmark must state the configured terrain, radius, player count, and
   mutation density.
3. Results must include command lines and commit identifiers.
4. Benchmarks are planning evidence until an owner doc makes them a merge gate.
5. Synthetic benchmarks do not replace compose probes for protocol behavior.

## Deferred

- Contraption-heavy tick benchmarks.
- Multi-region split and merge pressure.
- Entity persistence pressure.
- Anvil import throughput.
