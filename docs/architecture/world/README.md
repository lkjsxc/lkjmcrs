# World

Use this subtree for chunk, region, and world-data contracts.

## Read This Section When

- You need terrain behavior.
- You need data structure ownership.
- You need the current storage boundary.

## Child Index

- [flat-world.md](flat-world.md): deterministic first world.
- [chunk-storage.md](chunk-storage.md): chunk representation and persistence boundary.
- [chunk-streaming.md](chunk-streaming.md): bounded player-driven chunk
  streaming.
- [item-entities.md](item-entities.md): dropped item entity ownership and
  pickup.
- [mutable-chunks.md](mutable-chunks.md): sparse in-memory block overrides.
- [persistent-overrides.md](persistent-overrides.md): `redb` storage for sparse
  chunk overrides.
- [region-index.md](region-index.md): compact coordinate and ownership index.
