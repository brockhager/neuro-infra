# Multi-stage build for Rust application
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/neuroswarm-node /app/nsd

# Create config directory
RUN mkdir -p /root/.neuroswarm

# Default config
COPY ns.conf /root/.neuroswarm/ns.conf

EXPOSE 8080

CMD ["./nsd", "start"]