FROM rust:1.87-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder ./app/target/release/app ./target/release/app
ENV PORT 8080
EXPOSE $PORT

ENTRYPOINT ["./target/release/app"]
