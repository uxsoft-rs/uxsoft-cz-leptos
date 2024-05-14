FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-armv7-unknown-linux-gnueabihf.tgz
RUN tar -xvf cargo-binstall-armv7-unknown-linux-gnueabihf.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y


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