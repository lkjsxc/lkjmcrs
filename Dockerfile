FROM rust:1.91-alpine AS builder

WORKDIR /workspace
RUN apk add --no-cache build-base musl-dev
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY config ./config
RUN cargo build --release

FROM alpine:3.22

RUN adduser -D -u 10001 lkjmcrs
RUN mkdir -p /app/data /data && chown -R lkjmcrs:lkjmcrs /app /data
USER lkjmcrs
WORKDIR /app
COPY --from=builder /workspace/target/release/lkjmcrs /usr/local/bin/lkjmcrs
COPY --from=builder /workspace/config ./config
EXPOSE 25565
ENTRYPOINT ["lkjmcrs"]
