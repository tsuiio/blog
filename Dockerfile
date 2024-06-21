ARG RUST_VERSION=1.78.0
ARG APP_NAME=tsuiiblog

FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME

WORKDIR /build

RUN apt-get update \
    && apt-get install -y \
    libssl-dev pkg-config libpq-dev

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF
set -e
cargo build --release
mv ./target/release/$APP_NAME /build/$APP_NAME
EOF

FROM debian:bookworm-slim AS final
ARG APP_NAME

ARG UID=991
ARG GID=991

RUN apt-get update \
    && apt-get install -y \
    openssl ca-certificates libpq5 \
    tini \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists

RUN groupadd -g "${GID}" blog \
    && useradd -l -u "${UID}" -g "${GID}" -m -d /blog blog
USER blog
WORKDIR /blog

COPY --chown=blog:blog --from=build /build/$APP_NAME /blog/ 

RUN mkdir conf

EXPOSE 3000

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/blog/tsuiiblog"]