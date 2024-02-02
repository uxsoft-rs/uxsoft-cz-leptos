FROM rust:alpine AS builder
WORKDIR /build

RUN apk update && \
	apk upgrade --no-cache && \
	apk add pkgconfig libressl-dev musl-dev npm

RUN rustup default nightly 
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos
RUN npm install tailwindcss -g

COPY . .

RUN npx tailwindcss -i style/tailwind.css -o style/generated.css --minify
RUN cargo leptos build --release


FROM alpine:3.18 AS runner
WORKDIR /var/www/app

RUN addgroup -S server && \
	adduser -S www-data -G server && \
	chown -R www-data:server /var/www/app

COPY --chown=www-data:server --from=builder /build/target/server/release/uxsoft-cz-leptos ./server/uxsoft-cz-leptos
COPY --chown=www-data:server --from=builder /build/target/front/wasm32-unknown-unknown/release/uxsoft-cz-leptos.wasm ./front/uxsoft-cz-leptos.wasm
COPY --chown=www-data:server --from=builder /build/target/site ./site

USER www-data

ENV LEPTOS_OUTPUT_NAME "uxsoft-cz-leptos"
ENV LEPTOS_SITE_ROOT "/var/www/app/site"
ENV LEPTOS_ENV "PROD"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./server/uxsoft-cz-leptos"]