# Purpose

## Goal

Create a Rust Minecraft Java Edition server that can grow into a large-scale
PaperMC/Folia-class platform while staying readable to LLM agents.

## Target Version

- Minecraft Java Edition `1.21.11`.
- Protocol version `774`.
- World data version `4671`.
- Data pack version `94.1`.
- Resource pack version `75.0`.
- Java compatibility baseline `21` for client/server ecosystem context.

## Intended Users

- LLM agents that read docs, implement code, and verify changes.
- Human maintainers who review through AI-assisted workflows.
- Later, Minecraft server operators who need high-scale SMP behavior.

## First Useful State

The first useful state is a playable skeleton:

- vanilla client can see the server in the server list,
- offline-mode login reaches play state,
- player enters a deterministic flat world,
- server ticks and keepalive behavior are observable,
- Docker Compose smoke verification proves the wire path.

## Non-Goals

- No Bukkit, Paper, or Folia plugin compatibility promise.
- No backward compatibility with early internal code.
- No copied Mojang server implementation.
- No public plugin API in the first milestone.
- No original gameplay systems until the basic server loop is credible.
