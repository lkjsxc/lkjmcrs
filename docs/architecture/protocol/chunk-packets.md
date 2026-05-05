# Chunk Packets

## Packet Names

Protocol `774` names play packet `0x2c`
`clientbound/minecraft:level_chunk_with_light`.

Use this name in code and documentation. Older `map_chunk` labels are too vague
for the current debugging boundary.

## First Milestone Sequence

The play bootstrap sends:

1. `chunk_batch_start`
2. one `level_chunk_with_light` packet for every chunk in the advertised
   chunk-cache radius
3. one `update_light` packet after each chunk
4. `chunk_batch_finished`

The advertised radius is authoritative. A radius of `2` means the initial
terrain batch is a `5x5` square centered on chunk `0,0`, for `25` chunks total.
Do not advertise a larger radius than the bootstrap sends.

The explicit `update_light` packet is intentionally retained during the current
join milestone even though `level_chunk_with_light` already carries light data.
Removing it is a separate evidence-backed simplification.

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

## Heightmap Data

The first milestone sends two heightmaps for each chunk:

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

The first milestone uses one biome value: `minecraft:plains` registry ID `0`.

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
