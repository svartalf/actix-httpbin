FROM rust:latest AS build

ARG REPO_URL="https://github.com/svartalf/actix-httpbin.git"
ARG WORKDIR="/usr/local/src"
ARG TARGET="/usr/local/src/target/release/actix-httpbin"

WORKDIR ${WORKDIR}
RUN git clone --depth 1 ${REPO_URL} ${WORKDIR}; \
    RUSTFLAGS="-C target-cpu=native" cargo build --release; \
    strip ${TARGET}

FROM debian:stretch AS run

ARG TARGET="/usr/local/src/target/release/actix-httpbin"
COPY --from=build ${TARGET} /usr/bin
EXPOSE 80

CMD ["/usr/bin/actix-httpbin", "-b", "0.0.0.0:80"]
