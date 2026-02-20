FROM registry.openanalytics.eu/proxy/library/rust:slim-trixie AS builder

WORKDIR /app
ARG RUSTUP_DEFAULT_TOOLCHAIN=1.92 \
    CARGO_LEPTOS_VERSION=0.2.34 \
    SCCACHE_VERSION=0.10.0 \
    RUST_ARCH=x86_64-unknown-linux-gnu

ENV CARGO_TARGET_DIR=docker-target \
    LEPTOS_HASH_FILES=true

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-make
ADD https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-$RUST_ARCH.tgz .

RUN tar -xvf cargo-binstall-$RUST_ARCH.tgz \
    && mv cargo-binstall /usr/local/cargo/bin \
    && rm -f cargo-binstall-$RUST_ARCH.tgz

RUN cargo binstall --no-confirm --force cargo-make --secure
RUN cargo binstall --no-confirm --force cargo-leptos --secure
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown

COPY ./ ./

RUN cargo make build-demo

# Runner image
FROM registry.openanalytics.eu/proxy/library/debian:trixie-slim
WORKDIR /app
# Copy the server binary to the /app directory
COPY --from=builder /app/demo/docker-target/release/overview /app/demo
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/demo/target/site /app/site
COPY --from=builder /app/demo/style /app/style
COPY --from=builder /app/demo/target/release/hash.txt /app/hash.txt

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_HASH_FILES=true
EXPOSE 8080

# Run the server
CMD ["/app/demo"]
