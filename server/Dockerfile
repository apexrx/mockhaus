FROM rust:1.88-slim-bookworm AS builder
WORKDIR /app

ENV SQLX_OFFLINE=true
COPY Cargo.toml Cargo.lock ./
COPY src src
COPY migrations migrations
COPY templates templates
COPY static static
COPY .sqlx .sqlx

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates sqlite3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mockhaus-server /app/mockhaus-server

COPY static /app/static

EXPOSE 7070

CMD ["/app/mockhaus-server"]
