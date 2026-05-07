# Online Auth Smoke

## Goal

Verify that online mode authenticates through the encrypted Java login path and
continues through encrypted configuration and play entry.

## Contract

1. `online-server` runs with `online_mode=true`.
2. `session-fixture` answers
   `/session/minecraft/hasJoined` for `OnlineProbe`.
3. The fixture URL is HTTP and therefore requires
   `allow_insecure_session_server=true` in verification config.
4. The probe sends login start, completes encryption response, and enables
   AES/CFB8 before reading login success.
5. The login success UUID must match the fixture profile UUID.
6. The probe sends login acknowledged over the encrypted stream.
7. The probe completes known-packs, registry, feature, and finish-config
   exchange over the encrypted stream.
8. The probe validates the encrypted play bootstrap and replies to the initial
   keepalive.

## Non-Goals

- Mojang service availability.
- Secure chat.
- Compression.
- Manual stock-client evidence.
