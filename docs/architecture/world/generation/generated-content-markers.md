# Generated Content Markers

## Goal

Define internal markers that describe generated-origin content without changing
the persistence boundary or protocol chunk format.

## Owns

- Marker names for generated terrain stages.
- Rules for attaching markers to generated columns or blocks.
- Rules for dropping markers when persisted overrides replace generated output.
- Debug and verification use of markers.

## Inputs

- Stage outputs from [macro-fields.md](macro-fields.md),
  [hydrology.md](hydrology.md), [oceans-and-coasts.md](oceans-and-coasts.md),
  [biome-pipeline.md](biome-pipeline.md),
  [surface-palettes.md](surface-palettes.md), and
  [surface-decorators.md](surface-decorators.md).
- Final generated chunk data before sparse overrides are applied.

## Outputs

- Internal marker metadata for tests, diagnostics, or generation handoff.
- No protocol-visible fields.
- No persisted section override values.

## Rules

1. Markers are generated metadata, not world state.
2. Markers must be deterministic when regenerated from the same inputs.
3. Markers must not be required to decode or send a chunk.
4. Persisted player edits take ownership over affected blocks and invalidate
   any generated marker for those positions.
5. Markers must not introduce new storage schema requirements unless a separate
   storage owner doc accepts that boundary.
6. Marker names should describe the producing stage, such as `riverbed`,
   `coast`, `surface_palette`, or `decorator_candidate`.

## Verification

- Unit tests may assert markers to explain why generated samples exist.
- Golden tests may include marker expectations only when they clarify stage
  ownership.
- Protocol smoke tests must pass without reading markers.

## Out Of Scope

- Client-visible debug blocks.
- Persisted metadata for player edits.
- Region indexes, save-file formats, and external map exports.
