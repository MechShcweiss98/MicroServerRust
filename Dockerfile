# FROM rust:1.70 AS builder

# WORKDIR /usr/src/qr-system-rust

# COPY Cargo.toml ./

# COPY . .

# RUN cargo build --release || (echo "Compilation Error" && exit 1)
# RUN ls -l /usr/src/qr-system-rust/target/release

# RUN chmod +x /usr/src/qr-system-rust/target/release/QR-SystemRust

# FROM debian:buster-slim

# RUN apt-get update && apt-get install -y \
#     libssl-dev \
#     pkg-config \
#     --fix-missing \
#     && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /usr/src/qr-system-rust/target/release/QR-SystemRust /usr/local/bin/QR-SystemRust

# CMD ["QR-SystemRust"]


# Usa la imagen oficial de Rust como base
FROM rust:1.74 AS builder

# Establecer el directorio de trabajo
WORKDIR /app

# Copiar los archivos del proyecto
COPY . .

# Configurar las dependencias para que se cacheen correctamente
RUN cargo build --release

# Imagen final mínima
FROM debian:bullseye-slim

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copiar el binario desde la imagen anterior
COPY --from=builder /app/target/release/MicroServerRust /usr/local/bin/microserver

# Copiar archivos de migración
COPY --from=builder /app/migration /app/migration

# Instalar wait-for-it
RUN apt-get update && apt-get install -y netcat && rm -rf /var/lib/apt/lists/*

# Variables de entorno
ENV DATABASE_URL=postgres://postgres:5599@db:5432/users

# Comando de inicio
CMD ["sh", "-c", "sleep 3 && /usr/local/bin/microserver"]

