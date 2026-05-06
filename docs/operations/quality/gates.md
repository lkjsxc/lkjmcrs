# Quality Gates

## Required Gates

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- docs topology validation
- line-limit validation
- live compose probes for protocol changes

## Acceptance

1. Static `verify` and relevant live probes pass in Docker Compose.
2. Docs and implementation agree.
3. Git commits are coherent and frequent.
4. Failures are fixed or documented as blockers.
