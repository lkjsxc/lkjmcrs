# Dependency Policy

## Current Slice

- Use first-party protocol code.
- Use Tokio for async runtime.
- Use small, well-maintained crates for CLI, logging, errors, and UUIDs.
- Avoid Minecraft server frameworks during the current slice.
- Keep packet framing and Minecraft protocol types first-party.
- Keep the current network NBT writer first-party while the candidate
  `simdnbt` releases require nightly Rust in the stable verify image.
- Use bundled SQLite for player persistence before cross-player systems need a
  larger external database.

## Rules

1. Dependencies must serve a concrete implementation need.
2. Avoid crates that lag the exact target protocol for core compatibility.
3. Prefer stable Rust unless a doc explicitly approves nightly.
4. Record major dependency strategy changes in docs before code.
