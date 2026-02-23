FROM rust:1.89-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin app --bin run_daily_job

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app ./app
COPY --from=builder /app/target/release/run_daily_job ./run_daily_job
ENV PORT=8080
EXPOSE $PORT

ENTRYPOINT ["./app"]
