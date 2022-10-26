# Build project
FROM rust:latest as builder
WORKDIR /volume
COPY . .
RUN env RUSTFLAGS="-C target-cpu=haswell" cargo build --profile=release

# Setup actual executing image
FROM debian
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /volume/target/release/as2c /as2c
RUN mkdir /files

ENTRYPOINT ["/as2c"]
