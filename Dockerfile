# syntax=docker/dockerfile:1
FROM rust:1.93 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -m appuser
WORKDIR /data
COPY --from=builder /app/target/release/agent-data-pipeline-rust /usr/local/bin/pipeline
USER appuser
ENTRYPOINT ["/usr/local/bin/pipeline"]
