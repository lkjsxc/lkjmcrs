# Local Compose

## Service

`server` is the product runtime service.

## Defaults

- Image is built from the local Dockerfile.
- Container port is `25565`.
- Host port is `${LKJMCRS_PORT:-25565}`.
- Working mode is offline by default.

## Rules

1. Compose runtime must use the same binary built by release Dockerfile.
2. Verification may use separate cache volumes.
3. Runtime state remains disposable in the first milestone.
