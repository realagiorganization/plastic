# syntax=docker/dockerfile:1.7

ARG RUST_VERSION=1.84
ARG DEFAULT_VARIANT=ui

FROM rust:${RUST_VERSION}-slim AS builder
ARG DEFAULT_VARIANT
ENV CARGO_TERM_COLOR=never

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    clang \
    make \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /src

COPY . .

RUN cargo build --release --bin plastic --bin plastic_tui

FROM debian:bookworm-slim AS runtime
ARG DEFAULT_VARIANT
ENV PLASTIC_VARIANT=${DEFAULT_VARIANT}

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    libasound2 \
    libudev1 \
    libxcb-render0 \
    libxcb-shape0 \
    libxcb-xfixes0 \
    libxkbcommon0 \
    libgl1 \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/plastic /usr/local/bin/plastic
COPY --from=builder /src/target/release/plastic_tui /usr/local/bin/plastic_tui
COPY docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
