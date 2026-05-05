# Quality Gates

## Required Gates

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- docs topology validation
- line-limit validation
- live smoke probe for protocol changes

## Acceptance

1. Relevant gates pass in Docker Compose.
2. Docs and implementation agree.
3. Git commits are coherent and frequent.
4. Failures are fixed or documented as blockers.
