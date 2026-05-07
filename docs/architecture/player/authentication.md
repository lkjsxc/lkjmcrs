# Authentication

## Implemented Offline Mode

- Player name is validated for basic length and allowed characters.
- UUID is deterministic from `OfflinePlayer:<name>`.
- No Mojang session request is made.
- Offline-mode runtime is private-only when reachable players are not trusted.
- Exposure rules live in
  [../../operations/deployment/exposure-policy.md](../../operations/deployment/exposure-policy.md).

## Implemented Online Mode

- `online_mode=true` performs the Java login encryption handshake before login
  success.
- The server sends an empty server ID, a process-local RSA public key, a random
  verify token, and `should_authenticate=true`.
- The client response is RSA-decrypted; token mismatch disconnects login with
  `Authentication failed`.
- AES/CFB8 encryption is enabled after the server accepts the shared secret.
- The server hash covers the empty server ID, shared secret, and public key.
- `{session_server_url}/session/minecraft/hasJoined` is the verifier boundary.
- Only a successful JSON profile response is accepted.
- The returned UUID is authoritative for storage, session identity, and
  operator checks.
- Verifier failures and verifier timeout disconnect login with
  `Authentication failed`.
- Compression and secure chat are out of scope for this slice.

## Rules

1. Auth mode is configured at startup.
2. Login rejects unsupported protocol before creating a session.
3. Offline UUID generation is covered by tests.
4. Online verifier work stays outside tick and region workers.
5. Operator permission is UUID-based.
