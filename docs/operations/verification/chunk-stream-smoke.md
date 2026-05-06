# Chunk Stream Smoke

## Goal

Prove that movement across one chunk-center boundary unloads leaving chunks,
loads newly visible chunks, and keeps those chunks mutable through the normal
wire path.

## Scenario

1. Start the server through Docker Compose.
2. Log in and complete the normal play bootstrap.
3. Send a valid `position_look` movement from center `0,0` to center `1,0`.
4. Validate one `chunk_cache_center` update for `1,0`.
5. Validate exactly `5` unload packets for the old column `x=-2`.
6. Validate one chunk batch with exactly `5` chunks in the new column `x=3`.
7. Validate every streamed `level_chunk_with_light` and `update_light` payload.
8. Acquire dirt in one newly streamed chunk, place it, and observe the normal
   prediction acknowledgement and block update.

## Assertions

- The probe uses protocol packets only and does not inspect private server
  internals.
- Unload packets are expected for chunks leaving the configured visible window.
- Failure blocks acceptance for bounded chunk streaming or observer
  subscription updates.
