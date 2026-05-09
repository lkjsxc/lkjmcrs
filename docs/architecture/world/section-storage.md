# Section Storage

## Goal

Store sparse world overrides in a compact server-owned format while generated
terrain remains the base.

## Current Format

- Backend: `redb` database `world.redb`.
- Table: `chunk_sections`.
- Key: `overworld/{chunk_x}/{chunk_z}/{section_y}`.
- Meta marker: key `world_storage_schema`, value
  `lkjmcrs.section_overrides.current`.
- Value: binary section override record.

## Binary Value

All integers are little-endian.

1. Magic tag: `LKJMCRSS`.
2. Format marker byte: `1`.
3. Chunk `x`: signed `i32`.
4. Chunk `z`: signed `i32`.
5. Section `y`: signed `i32`.
6. Override count: unsigned `u16`.
7. Repeated override records sorted by `(local_x, local_y, local_z)`:
   local `x` as `u8`, local `y` as `u8`, local `z` as `u8`, block state code
   as `u16`.

## Validation

- Chunk and section coordinates in the value must match the table key owner.
- Local `x`, `y`, and `z` must be `0..15`.
- Absolute `section_y * 16 + local_y` must be inside the encoded world height.
- Block state codes must resolve to the current world block palette.
- Duplicate override positions are invalid.
- Truncated or trailing bytes are invalid.

## Current Block State Codes

- `0`: `minecraft:air`.
- `1`: `minecraft:bedrock`.
- `2`: `minecraft:stone`.
- `3`: `minecraft:dirt`.
- `4`: `minecraft:grass_block`.

## Rules

- Generated terrain remains outside persistence unless a later owner doc
  explicitly defines full chunk storage.
- Empty sections must not keep table values.
- Anvil import is later work and must not change the `WorldStore` boundary.
