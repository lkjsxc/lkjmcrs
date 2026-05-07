# Chunk Packets

## Packet Names

Protocol `774` names play packet `0x2c` `clientbound/minecraft:map_chunk` in
the generated packet table. This project uses the more descriptive local name
`level_chunk_with_light` because the payload includes chunk data and light data.

Use this name in code and documentation. Older `map_chunk` labels are too vague
for the current debugging boundary.

## Current Join Sequence

The play bootstrap sends:

1. `chunk_batch_start`
2. one `level_chunk_with_light` packet for every chunk in the advertised
   chunk-cache radius
3. `chunk_batch_finished`

The advertised radius is authoritative. A radius of `2` means the initial
terrain batch is a `5x5` square centered on chunk `0,0`, for `25` chunks total.
Do not advertise a larger radius than the bootstrap sends.

`level_chunk_with_light` carries the complete light payload for the current flat
chunk. Do not send a separate `update_light` packet during bootstrap or normal
movement chunk batches. Reserve explicit `update_light` for later changed-light
events after the server has mutable lighting.

## Level Chunk With Light Payload

The packet payload is:

1. chunk X as signed big-endian `i32`
2. chunk Z as signed big-endian `i32`
3. heightmap data
4. VarInt byte length for chunk section data
5. raw chunk section data
6. VarInt block entity count
7. light data

The chunk section byte length covers only the raw section data.
It excludes heightmap data, block entity count/data, and light data.

## Heightmap Data

The current join sequence sends two heightmaps for each chunk:

- `WORLD_SURFACE`,
- `MOTION_BLOCKING`.

Each heightmap contains `256` entries with `9` bits per entry. The packed long
array uses fixed values-per-long storage:

- `values_per_long = floor(64 / 9) = 7`,
- values do not cross long boundaries,
- `raw_long_count = ceil(256 / 7) = 37`.

Compact bit-stream packing would produce `36` longs and is rejected by the
client with `expected: 37, got: 36`.

## Section Data

Each vertical section writes:

1. non-air block count as unsigned big-endian `u16`
2. block state paletted container for `4096` entries
3. biome paletted container for `64` entries

The current join sequence uses one biome value: `minecraft:plains` registry ID
`0`.

## Paletted Container Wire Shape

Section paletted containers write:

1. bits per entry as one unsigned byte
2. palette payload
3. fixed-size raw long array

They do not write a VarInt long-array length. The client derives the raw array
size from container kind and bits per entry.

For bits per entry `0`:

- palette payload is one VarInt value
- raw long array length is zero

For indirect palettes with bits per entry greater than zero:

- palette payload is a VarInt palette length followed by palette VarInt values
- `values_per_long = floor(64 / bits_per_entry)`
- `raw_long_count = ceil(entry_count / values_per_long)`
- values do not cross long boundaries

## Regression Anchors

The historical `readerIndex(6345) + length(8) exceeds writerIndex(6345)` crash
came from a malformed chunk-section stream. The current slice must keep tests
that prove the chunk-data byte range is consumed exactly.

For the current flat spawn chunk, the encoded chunk section data length is
`6294` bytes. If that length changes, update this file and the tests in the
same batch with the new documented reason.
