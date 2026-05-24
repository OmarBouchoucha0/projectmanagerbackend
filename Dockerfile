FROM rust:1.89 AS builder

WORKDIR /app

COPY . .
RUN cargo build --release
RUN ls -lah /app/target/release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/projectmanagerbackend .

EXPOSE 5000

CMD ["./projectmanagerbackend"]
