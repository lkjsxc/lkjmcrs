# Research Disposition

## Goal

Define how agents may use terrain research reports without changing the
first-party Rust server target by accident.

## Accepted Lessons

- Deterministic terrain must be reproducible from the seed, absolute
  coordinates, and a named formula marker.
- Macro fields should decide landmass, uplift, coast, water, and broad biome
  hints before local surface painting.
- Hydrology and coast shape should be first-class generated stages, not
  afterthoughts pasted over a finished height field.
- Surface decorators should use deterministic spacing, slope, substrate,
  water, and headroom checks.
- Verification should combine golden samples, border properties, live chunk
  probes, large-radius streaming, and current evidence notes.

## Rejected Runtime Direction

- Paper, Bukkit, Folia, Terra, Tectonic, Terralith, Chunky, spark, and map
  tooling are not runtime dependencies for this repository.
- No plugin, datapack, Anvil, or Terra config-pack support is introduced by
  citing those projects.
- External projects may inspire local contracts only after their lessons are
  rewritten into `docs/` using first-party Rust server vocabulary.

## Current Application

- The active target remains surface-first normal survival terrain.
- The first promoted decorator family is spruce-style wood terrain using
  vanilla block states owned by protocol and storage docs.
- The first promoted quality gate is a live `terrain-quality` probe requiring
  dry spawn footing, nearby water, generated wood, and non-flat terrain.

## Rules

1. `tmp/deep-research-report*.md` files are input notes, not canon.
2. Useful research must be copied into owner docs before implementation.
3. Rejected runtime directions must not appear in code, configs, or compose
   services.
4. Research-driven behavior still needs Docker Compose verification before it
   is recorded as current capability.
