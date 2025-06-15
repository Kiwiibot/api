FROM rust:1.87.0 AS builder

WORKDIR /tmp

COPY Cargo.toml Cargo.lock /tmp/
COPY image_derive /tmp/image_derive
COPY images /tmp/images

RUN cargo build --release --bin kiwii_api

FROM debian:bookworm-slim AS app

EXPOSE 5555

WORKDIR /app

COPY --from=builder /tmp/target/release/kiwii_api /app/kiwii_api

RUN apt-get update \
  && apt-get install -y --no-install-recommends openssl fontconfig \
  && fc-cache -fv \
  && rm -rf /var/lib/apt/lists/*

CMD ["/app/kiwii_api"]
