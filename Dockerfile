FROM rust:1-bookworm AS builder

ARG CARGO_LEPTOS_VERSION=0.3.6

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates clang curl libsqlite3-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -LsSf \
    "https://github.com/leptos-rs/cargo-leptos/releases/download/v${CARGO_LEPTOS_VERSION}/cargo-leptos-installer.sh" \
    | sh

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .

RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/alex-hou-2024-test-16 /app/alex-hou-2024-test-16
COPY --from=builder /app/target/site /app/site

RUN mkdir -p /app/data

ENV DATABASE_URL=sqlite:///app/data/todos.db
ENV LEPTOS_OUTPUT_NAME=alex-hou-2024-test-16
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV LEPTOS_SITE_ROOT=/app/site

VOLUME ["/app/data"]

EXPOSE 8080

CMD ["/app/alex-hou-2024-test-16"]
