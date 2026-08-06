FROM rust:1.97 AS builder
WORKDIR /app

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler
RUN ls /usr/bin/pr*
ENV PROTOC=/usr/bin/protoc

COPY . .
RUN cargo build --release

FROM debian:stable-slim AS runtime
RUN apt-get update && \
    apt-get install pkg-config libssl-dev ca-certificates -y && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/drm-beatsaver-cacher /usr/local/bin
CMD ["drm-beatsaver-cacher"]