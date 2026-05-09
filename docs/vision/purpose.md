# Purpose

## Goal

Create a Rust Minecraft Java Edition server that can grow into a large-scale
PaperMC/Folia-class platform while staying readable to LLM agents.

## Target Release

- Minecraft Java Edition `1.21.11`.
- Protocol number `774`.
- World data number `4671`.
- Data pack number `94.1`.
- Resource pack number `75.0`.
- Java compatibility baseline `21` for client/server ecosystem context.

## Intended Users

- LLM agents that read docs, implement code, and verify changes.
- Human maintainers who review through AI-assisted workflows.
- Later, Minecraft server operators who need high-scale SMP behavior.

## First Useful State

The first useful state is a playable server:

- vanilla client can see the server in the server list,
- first-party wire probe reaches offline login and play state,
- player enters deterministic generated terrain with a protected spawn safety
  core,
- server ticks and keepalive behavior are observable,
- Docker Compose smoke verification proves the wire path.

## Non-Goals

- No Bukkit, Paper, or Folia plugin compatibility promise.
- No backward compatibility with early internal code.
- No copied Mojang server implementation.
- No public plugin API in the current slice.
- No original gameplay systems until the basic server loop is credible.
