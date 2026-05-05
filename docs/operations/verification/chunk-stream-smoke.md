# Chunk Stream Smoke

## Goal

Prove that movement across one chunk-center boundary loads and sends only the
newly visible chunks, and that those chunks become mutable through the normal
wire path.

## Scenario

1. Start the server through Docker Compose.
2. Log in and complete the normal play bootstrap.
3. Send a valid `position_look` movement from center `0,0` to center `1,0`.
4. Validate one `chunk_cache_center` update for `1,0`.
5. Validate one chunk batch with exactly `5` chunks in the new column `x=3`.
6. Validate every streamed `level_chunk_with_light` and `update_light` payload.
7. Place fixed stone in one newly streamed chunk and observe the normal
   prediction acknowledgement and block update.

## Assertions

- The probe uses protocol packets only and does not inspect private server
  internals.
- No client unload packet is expected or accepted in this milestone.
- Failure blocks acceptance for movement-driven chunk loading or observer
  subscription changes.
