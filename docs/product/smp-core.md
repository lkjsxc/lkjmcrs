# SMP Core

## Goal

Make multiplayer sessions operable before adding deeper survival or original
gameplay systems.

## First Slice

- Offline-mode identities remain authoritative.
- Plain unsigned chat is accepted and broadcast as server-authored system chat.
- Slash commands use the serverbound command packets, not chat text with a
  leading slash.
- The server sends a small command tree during play bootstrap.
- New `LKJMCRS_OPS` config grants operator permission by player name.
- Operator names are matched case-insensitively.

## Commands

All players may use:

- `/help`: list supported commands.
- `/spawn`: teleport the caller to `0.5, 80.0, 0.5`.

Operators may use:

- `/say <message>`: broadcast a server-authored message.
- `/gamemode <survival|creative> [player]`: change the caller or target mode.
- `/kick <player> [reason]`: disconnect a connected player.

## Rules

1. Commands never mutate world chunks directly.
2. Gamemode changes update abilities immediately and persist on disconnect.
3. Kick sends a play disconnect packet with a text reason.
4. Unknown or unauthorized commands return system chat only to the caller.
5. Chat signing, secure profiles, and online-mode identity are out of scope.
