# Profile Reconnect Smoke

## Goal

Prove that player position and look state survive disconnect and reconnect
through the public wire path.

## Scenario

1. Start the server through Docker Compose.
2. Log in as one offline player.
3. Complete configuration and play bootstrap.
4. Send a valid movement packet with non-default position and look values.
5. Disconnect the client.
6. Reconnect with the same offline player name.
7. Validate the initial position packet uses the saved values.

## Assertions

- The probe does not inspect private server internals.
- The saved profile lives in `data_dir/players.sqlite3`.
- Existing chunk override persistence remains covered by
  [persistence-smoke.md](persistence-smoke.md).
- Failure in this probe blocks acceptance for player storage or bootstrap
  changes.
