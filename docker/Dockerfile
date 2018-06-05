FROM rust:latest AS build

ARG REPO_URL="https://github.com/svartalf/actix-httpbin.git"
ARG WORKDIR="/usr/local/src"
ARG TARGET="/usr/local/src/target/release/actix-httpbin"

WORKDIR ${WORKDIR}
RUN set -eux; \
    apt-get update; \
    apt-get install -y musl-dev musl-tools; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*; \
    rustup target add x86_64-unknown-linux-musl; \
    rustup target list; \
    git clone --depth 1 ${REPO_URL} ${WORKDIR}; \
    RUSTFLAGS="-C target-cpu=native" cargo build --target x86_64-unknown-linux-musl --release

FROM busybox:latest AS run

ARG TARGET="/usr/local/src/target/release/actix-httpbin"
COPY --from=build ${TARGET} /
EXPOSE 80

CMD ["/actix-httpbin"]
# , "-b", "0.0.0.0:80"]
