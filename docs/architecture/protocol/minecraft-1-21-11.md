# Minecraft 1.21.11 Contract

## Constants

- Client release name: `1.21.11`.
- Protocol number: `774`.
- World data version: `4671`.
- Data pack version: `94.1`.
- Resource pack version: `75.0`.
- Java runtime ecosystem baseline: `21`.

## Source Policy

These constants were checked against Mojang version metadata and `version.json`
from the official `1.21.11` server jar on `2026-05-05`.

## Rules

1. Server-list status advertises protocol `774`.
2. Login accepts only protocol `774`.
3. Unsupported protocol numbers are disconnected before play state.
4. Protocol constants live in code as named constants.
5. Any Minecraft target change requires this file, tests, and smoke probes to
   change.
