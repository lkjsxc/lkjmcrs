# World

Use this subtree for chunk, region, and world-data contracts.

## Read This Section When

- You need terrain behavior.
- You need data structure ownership.
- You need the current storage boundary.

## Terrain And Spawn

- [terrain-pipeline.md](terrain-pipeline.md): generated terrain, overrides,
  and chunk encoding.
- [terrain-generation.md](terrain-generation.md): current generator behavior.
- [water-and-rivers.md](water-and-rivers.md): static water and river terrain.
- [caves.md](caves.md): generated underground cave terrain.
- [spawn-resolution.md](spawn-resolution.md): default spawn, respawn, `/spawn`,
  and teleport safety.
- [flat-world.md](flat-world.md): controlled flat generator.

## Chunk Data And Storage

- [chunk-storage.md](chunk-storage.md): chunk representation and persistence
  boundary.
- [mutable-chunks.md](mutable-chunks.md): sparse in-memory block overrides.
- [persistent-overrides.md](persistent-overrides.md): `redb` sparse override
  behavior.
- [section-storage.md](section-storage.md): binary section override value.
- [storage-schema.md](storage-schema.md): `redb` world tables and keys.
- [storage-migration.md](storage-migration.md): unsupported early formats.

## Streaming And Runtime Ownership

- [chunk-streaming.md](chunk-streaming.md): bounded player-driven streaming.
- [large-distance-streaming.md](large-distance-streaming.md): progressive
  larger-radius delivery.
- [region-index.md](region-index.md): compact coordinate and ownership index.
- [item-entities.md](item-entities.md): dropped item entity ownership.
