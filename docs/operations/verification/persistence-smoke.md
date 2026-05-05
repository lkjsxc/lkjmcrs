# Persistence Smoke

## Goal

Prove that an accepted block mutation survives a server restart through the
configured data directory.

## Scenario

1. Start the server through Docker Compose.
2. Run a probe that logs in, places fixed stone at `0,80,0`, and disconnects.
3. Restart the server without removing compose volumes.
4. Run a probe that logs in and inspects the bootstrap chunk containing
   `0,80,0`.
5. The probe must observe stone at `0,80,0` before sending new mutations.

## Assertions

- The persisted block is encoded in the normal `level_chunk_with_light` payload.
- The check does not rely on private server internals.
- `down -v` remains the command that removes persisted compose state.
