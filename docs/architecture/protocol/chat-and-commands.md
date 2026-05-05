# Chat And Commands

## Source Baseline

Packet IDs and shapes are pinned to `minecraft-data` `1.21.11` protocol `774`.

## Clientbound Packets

- `0x10 declare_commands`: small command tree for supported slash commands.
- `0x20 kick_disconnect`: anonymous NBT text component reason.
- `0x26 game_state_change`: reused for gamemode change event `3`.
- `0x3e abilities`: resent after gamemode changes.
- `0x46 position`: reused by `/spawn` with absolute flags.
- `0x77 system_chat`: anonymous NBT text component plus action-bar flag.

## Serverbound Packets

- `0x06 chat_command`: unsigned slash command without leading slash.
- `0x07 chat_command_signed`: signed command envelope; only command text is
  interpreted in offline mode.
- `0x08 chat_message`: unsigned text is broadcast as system chat.
- `0x34 held_item_slot`: signed 16-bit hotbar slot.

## Command Tree

The first tree declares root literals for:

- `help`
- `spawn`
- `say`
- `gamemode`
- `kick`

Arguments are kept permissive and server-validated. The tree exists so vanilla
clients can send command packets normally; server-side parsing remains
authoritative.

## Rules

1. Client chat is never forwarded as signed `player_chat`.
2. System messages use text components shaped as anonymous NBT compounds.
3. Decoders reject trailing bytes in tests.
4. Unsupported signed metadata is consumed but not trusted.
