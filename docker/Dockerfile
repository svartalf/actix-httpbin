FROM rust:latest AS build

ARG REPO_URL="https://github.com/svartalf/actix-httpbin.git"
ARG WORKDIR="/usr/local/src"
ARG TARGET="/usr/local/src/target/release/actix-httpbin"

WORKDIR ${WORKDIR}
RUN git clone -q --depth 1 ${REPO_URL} ${WORKDIR}; \
    cargo build --release; \
    strip ${TARGET}

FROM alpine:latest AS run

ARG TARGET="/usr/local/src/target/release/actix-httpbin"

COPY --from=build ${TARGET} /bin
RUN ls -la /bin
EXPOSE 80

CMD ["actix-httpbin -p 80"]
