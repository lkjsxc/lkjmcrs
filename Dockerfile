FROM rust:1.91-alpine AS builder

WORKDIR /workspace
RUN apk add --no-cache build-base musl-dev
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.22

RUN adduser -D -u 10001 lkjmcrs
USER lkjmcrs
WORKDIR /app
COPY --from=builder /workspace/target/release/lkjmcrs /usr/local/bin/lkjmcrs
EXPOSE 25565
ENTRYPOINT ["lkjmcrs", "serve"]
