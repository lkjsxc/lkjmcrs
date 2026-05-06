# Packet Contract

## Login

Target protocol: `774` for Minecraft Java Edition `1.21.11`.

- `login/clientbound 0x02 success`: UUID, username string, property array.
- The property array is VarInt-counted entries of name, value, optional
  signature.
- There is no trailing boolean after the property array.
- `login/serverbound 0x03 login_acknowledged` has an empty payload.

## Configuration

- `0x0e select_known_packs`: sends `minecraft:core` version `1.21.11`.
- `0x07 registry_data`: one packet per declared dynamic registry.
- `0x0d tags`: tag groups for declared dynamic registries.
- `0x0c feature_flags`: sends `minecraft:vanilla`.
- `0x03 finish_configuration`: empty payload.

The first milestone declares one registry packet for each of these registries:

- `minecraft:dimension_type` with `minecraft:overworld` at registry ID `0`.
- `minecraft:worldgen/biome` with `minecraft:plains` at registry ID `0`.
- `minecraft:damage_type` with the bootstrap keys in
  [dynamic-registries.md](dynamic-registries.md).
- Minimal non-empty variant registries required by the vanilla client:
  cat, chicken, cow, frog, painting, pig, wolf sound, wolf, and zombie nautilus.
- `minecraft:timeline` with `minecraft:day` at registry ID `0`.
- `minecraft:timeline` tag `minecraft:in_overworld` binds to ID `0`.

## Play Bootstrap

- `0x30 login`: one world, `minecraft:overworld`; view distance defaults to
  `2`, and simulation distance defaults to the same value.
- `0x5f spawn_position`: global position in `minecraft:overworld`.
- `0x6f update_time`: age `0`, time `0`, ticking enabled.
- `0x3e abilities`: permissive initial ability flags.
- `0x26 game_state_change`: event `13`, `start_waiting_for_level_chunks`, value
  `0.0`.
- `0x5c update_view_position`: spawn chunk `0,0`.
- `0x5d set_chunk_cache_radius`: configured view distance.
- `0x67 held_item_slot`: authoritative selected hotbar slot.
- `0x6a set_player_inventory`: player inventory slot ID and slot contents.
- `0x0c chunk_batch_start`: empty payload.
- `0x2c level_chunk_with_light`: flat chunks for the configured view distance,
  with chunk data and light arrays.
- `0x2f update_light`: explicit light data for the same chunk, retained for the
  current join milestone.
- `0x0b chunk_batch_finished`: batch size.
- `0x46 position`: spawn teleport with teleport ID `1`.
- `0x2b keep_alive`: signed 64-bit keepalive ID.

The `level_chunk_with_light` count is derived from
`(radius * 2 + 1) ^ 2`. The default radius is `2`, so the bootstrap
must send `25` chunks. A smaller `3x3` batch is invalid because it advertises
terrain the client never receives during initial world entry.

Movement may send `0x25 unload_chunk` for chunks leaving the visible window.
Its payload is `chunkZ` as `i32`, then `chunkX` as `i32`.

The game-state change is a readiness gate, not cosmetic state. A modern vanilla
client can remain on terrain loading even after receiving chunks if the server
never sends event `13`. The packet ID must be `0x26` for protocol `774`; `0x21`
is chat and will be decoded as `clientbound/minecraft:disguised_chat` or
`profileless_chat`.

## Flat Chunk IDs

Default block-state IDs are pinned from `minecraft-data` `1.21.11`:

- air: `0`
- stone: `1`
- grass block default: `9`
- dirt: `10`
- bedrock: `85`

## Block Interaction

The first Survival Core slice adds creative-style mutation packets:

- `0x04 block_changed_ack`: acknowledges a client prediction sequence.
- `0x08 block_update`: sends one packed position and one block-state ID.
- `0x28 player_action`: starts or stops block breaking.
- `0x3c swing`: accepted and ignored.
- `0x3f use_item_on`: places fixed stone beside the targeted face.

See [block-interaction.md](block-interaction.md) for payload details.

## Chat And Commands

The first SMP slice adds offline-mode control packets:

- `0x06 serverbound chat_command`: command text without leading slash.
- `0x07 serverbound chat_command_signed`: command text plus ignored signature
  envelope.
- `0x08 serverbound chat_message`: plain message text plus ignored signature
  envelope.
- `0x10 clientbound declare_commands`: minimal command tree.
- `0x20 clientbound kick_disconnect`: anonymous NBT text reason.
- `0x34 serverbound held_item_slot`: signed 16-bit selected slot.
- `0x77 clientbound system_chat`: anonymous NBT text plus action-bar flag.

See [chat-and-commands.md](chat-and-commands.md) for payload details.

## Inventory Projection

The Survival Sandbox uses the player-inventory path before full container
support:

- `0x5e set_cursor_item`: deferred until cursor interactions.
- `0x67 held_item_slot`: clientbound selected hotbar slot as VarInt.
- `0x6a set_player_inventory`: clientbound `slotId` VarInt plus `Slot`.
- `0x12 window_items`: deferred until full container support.
- `0x14 set_slot`: deferred until full container support.

Protocol `774` slot encoding for this slice:

- Empty slot writes VarInt `itemCount = 0`.
- Non-empty slot writes VarInt `itemCount`, VarInt `itemId`, VarInt
  `addedComponentCount = 0`, and VarInt `removedComponentCount = 0`.

## Item Entities

The dropped item slice uses these protocol `774` clientbound packets:

- `0x01 spawn_entity`: entity ID VarInt, UUID, type VarInt `71`, position
  `f64 x/y/z`, zero velocity vector, zero pitch/yaw/head pitch, object data
  VarInt `0`.
- `0x61 entity_metadata`: entity ID VarInt plus metadata entry index `8`, type
  `7 item_stack`, encoded `Slot`, and terminator byte `0xff`.
- `0x7a collect`: collected entity ID, collector entity ID, and item count.
- `0x4b entity_destroy`: VarInt-counted entity ID array.

Item entity packet facts are pinned from the same `minecraft-data` `1.21.11`
source as packet IDs. The item entity type ID is `71`.
