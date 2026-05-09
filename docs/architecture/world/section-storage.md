# Section Storage

## Goal

Store sparse world overrides in a compact server-owned format while generated
terrain remains the base.

## Current Format

- Backend: `redb` database `world.redb`.
- Table: `chunk_overrides`.
- Key: `overworld/{chunk_x}/{chunk_z}`.
- Meta marker: key `world_override_format`, value `lkjmcrs.chunk_overrides.v1`.
- Value: binary chunk override record.

## Binary Value

All integers are little-endian.

1. Magic tag: `LKJMCRSCO`.
2. Format marker byte: `1`.
3. Chunk `x`: signed `i32`.
4. Chunk `z`: signed `i32`.
5. Override count: unsigned `u16`.
6. Repeated override records sorted by `(local_x, y, local_z)`:
   local `x` as `u8`, absolute `y` as signed `i32`, local `z` as `u8`,
   block state code as `u16`.

## Validation

- Chunk coordinates in the value must match the table key owner.
- Local `x` and `z` must be `0..15`.
- `y` must be inside the encoded world height.
- Block state codes must resolve to the current world block palette.
- Duplicate override positions are invalid.
- Truncated or trailing bytes are invalid.

## Current Block State Codes

- `0`: `minecraft:air`.
- `1`: `minecraft:bedrock`.
- `2`: `minecraft:stone`.
- `3`: `minecraft:dirt`.
- `4`: `minecraft:grass_block`.

## Future Section Direction

- Section-local records may replace whole-chunk records when mutation density
  or region ownership needs it.
- Generated terrain remains outside persistence unless a later owner doc
  explicitly defines full chunk storage.
- Anvil import is a later compatibility feature and must not change the
  `WorldStore` boundary.
