FROM lukemathwalker/cargo-chef:latest-rust-1.97.1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && \
    apt-get install protobuf-compiler -y
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
RUN apt-get update && \
    apt-get install pkg-config libssl-dev ca-certificates -y && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/drm-beatsaver-cacher /usr/local/bin
CMD ["drm-beatsaver-cacher"]