FROM rustlang/rust:nightly-bookworm as builder
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
COPY --from=builder /app/target/release/site /app/bin
COPY --from=builder /app/target/site /app/site
WORKDIR /app

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080

EXPOSE 8080
ENTRYPOINT ["/app/bin"]
