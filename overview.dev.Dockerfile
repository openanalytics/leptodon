FROM node:22-bookworm-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

ARG RUSTUP_DEFAULT_TOOLCHAIN=1.92 \
    CARGO_LEPTOS_VERSION=0.2.34 \
    SCCACHE_VERSION=0.10.0 \
    RUST_ARCH=x86_64-unknown-linux-gnu

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        pkg-config \
        libssl-dev \
        clang \
        ; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/dist/${rustArch}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUSTUP_DEFAULT_TOOLCHAIN; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    \
    apt-get remove -y --auto-remove \
        wget \
        ; \
    rm -rf /var/lib/apt/lists/*;
# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
ADD https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-$RUST_ARCH.tgz .
RUN tar -xvf cargo-binstall-$RUST_ARCH.tgz \
  && mv cargo-binstall /usr/local/cargo/bin \
  && rm -f cargo-binstall-$RUST_ARCH.tgz

# Install cargo-leptos
RUN cargo binstall cargo-leptos@$CARGO_LEPTOS_VERSION -y
RUN rustup target add wasm32-unknown-unknown
ADD https://github.com/mozilla/sccache/releases/download/v$SCCACHE_VERSION/sccache-v$SCCACHE_VERSION-x86_64-unknown-linux-musl.tar.gz /tmp
RUN tar zxvf /tmp/sccache-v$SCCACHE_VERSION-x86_64-unknown-linux-musl.tar.gz && \
    mv sccache-v$SCCACHE_VERSION-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache && \
    rm -r sccache-v$SCCACHE_VERSION-x86_64-unknown-linux-musl && \
    rm /tmp/sccache-v$SCCACHE_VERSION-x86_64-unknown-linux-musl.tar.gz && \
    chmod +x /usr/local/bin/sccache
