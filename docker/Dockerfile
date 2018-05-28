FROM rust:latest AS build

ARG REPO_URL="https://github.com/svartalf/actix-httpbin.git"
ARG WORKDIR="/usr/local/src"
ARG TARGET="/usr/local/src/target/release/actix-httpbin"

WORKDIR ${WORKDIR}
RUN git clone -q --depth 1 ${REPO_URL} ${WORKDIR}; \
    RUSTFLAGS="-C target-cpu=native" cargo build --release --quiet; \
    strip ${TARGET}

FROM alpine:latest AS run

ARG TARGET="/usr/local/src/target/release/actix-httpbin"
COPY --from=build ${TARGET} /bin
EXPOSE 80

CMD ["/bin/actix-httpbin -p 80"]
