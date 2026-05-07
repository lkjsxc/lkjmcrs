# Online Auth Smoke

## Goal

Verify that online-mode login uses the encrypted Java login path and accepts the
UUID returned by the session verifier.

## Contract

1. `online-server` runs with `online_mode=true`.
2. `session-fixture` answers
   `/session/minecraft/hasJoined` for `OnlineProbe`.
3. The fixture URL is HTTP and therefore requires
   `allow_insecure_session_server=true` in verification config.
4. The probe sends login start, completes encryption response, enables
   AES/CFB8, and reads encrypted login success.
5. The login success UUID must match the fixture profile UUID.

## Non-Goals

- Mojang service availability.
- Secure chat.
- Compression.
- Full play bootstrap after online login success.
