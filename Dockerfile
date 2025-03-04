FROM rust:1.70 AS builder

WORKDIR /usr/src/qr-system-rust

COPY Cargo.toml ./

COPY . .

RUN cargo build --release || (echo "Compilation Error" && exit 1)
RUN ls -l /usr/src/qr-system-rust/target/release

RUN chmod +x /usr/src/qr-system-rust/target/release/QR-SystemRust

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    --fix-missing \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/qr-system-rust/target/release/QR-SystemRust /usr/local/bin/QR-SystemRust

CMD ["QR-SystemRust"]
