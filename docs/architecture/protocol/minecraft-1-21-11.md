# Minecraft 1.21.11 Contract

## Constants

- Client release name: `1.21.11`.
- Protocol number: `774`.
- World data marker: `4671`.
- Data pack marker: `94.1`.
- Resource pack marker: `75.0`.
- Java runtime ecosystem baseline: `21`.

## Source Policy

This file owns the Minecraft target constants for docs and code. Other docs
should link here instead of repeating this table unless the exact value is part
of a packet, probe, or config contract.

These constants were checked against Mojang release metadata and the official
server jar metadata file for `1.21.11` on `2026-05-05`.

## Rules

1. Server-list status advertises protocol `774`.
2. Login accepts only protocol `774`.
3. Unsupported protocol numbers are disconnected before play state.
4. Protocol constants live in code as named constants.
5. Any Minecraft target change requires this file, tests, and smoke probes to
   change.
