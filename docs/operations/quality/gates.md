# Quality Gates

## Required Gates

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- docs topology validation
- line-limit validation
- live compose probes for protocol changes

## Static Output Contract

`docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify`
runs static gates through `scripts/verify-static.sh`.

On success, the `verify` service prints one summary line per gate and a final
pass line:

```text
verify fmt ... ok
verify clippy ... ok
verify test ... ok
verify docs-topology ... ok
verify line-limits ... ok
verify pass
```

On the first failure, `verify` prints the failed stage, dumps only that stage's
captured output, and exits non-zero:

```text
verify <stage> ... failed
----- <stage> output -----
<captured stdout/stderr from the failed command>
```

Compose lifecycle output may still appear outside the service output.

## Acceptance

1. Static `verify` and relevant live probes pass in Docker Compose.
2. Docs and implementation agree.
3. Git commits are coherent and frequent.
4. Failures are fixed or documented as blockers.
