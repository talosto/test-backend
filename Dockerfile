FROM rust:1.64-slim AS deps

WORKDIR /var
RUN USER=root cargo new app

WORKDIR /var/app
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN cargo build --release && rm -rf src && rm -rf target/release/$(grep -Po '(?<=^name = ")[^"]*(?=".*)' ./Cargo.toml)

# Сборка приложения
FROM deps AS build

WORKDIR /var/app
COPY src ./src/
RUN touch src/main.rs && cargo build --release
RUN mv ./target/release/$(grep -Po '(?<=^name = ")[^"]*(?=".*)' ./Cargo.toml) ./binfile

# Запуск приложения10 
FROM debian:stable-slim
RUN ln -s /usr/lib/libnsl.so.1 /usr/lib/libnsl.so.1.1
WORKDIR /var/app
COPY --from=build /var/app/binfile ./binfile
RUN chmod +x ./binfile
CMD ./binfile