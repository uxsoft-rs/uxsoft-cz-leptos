FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN cargo install --locked cargo-leptos


# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN npm install

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/uxsoft-cz-leptos /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV LEPTOS_SITE_ADDR "0.0.0.0:8090"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8090

CMD ["/app/uxsoft-cz-leptos"]