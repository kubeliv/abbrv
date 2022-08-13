FROM rust:1.62.1 as builder
WORKDIR /usr/src/abbrv
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch
COPY . .
RUN cargo install --path .

FROM debian:buster
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/abbrv /usr/local/bin/abbrv

ENV ROCKET_ADDRESS="0.0.0.0"
CMD ["abbrv"]
