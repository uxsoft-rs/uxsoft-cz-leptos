# FROM rustlang/rust:nightly-alpine as builder
FROM rust:alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall cargo-leptos -y

# Add the WASM target

WORKDIR /work
COPY . .

RUN npm install

RUN rustup target add wasm32-unknown-unknown

RUN cargo leptos build --release -vv

# FROM rustlang/rust:nightly-alpine as runner
FROM alpine:latest AS runner

WORKDIR /app

COPY --from=builder /work/target/release/uxsoft-cz-leptos /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV LEPTOS_SITE_ADDR "0.0.0.0:8090"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8090

CMD ["/app/uxsoft-cz-leptos"]
