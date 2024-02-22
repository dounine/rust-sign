FROM alpine:3.14
WORKDIR /app
RUN apk add --no-cache --virtual .build-deps curl gcc zip unzip openssl-dev openssl-libs-static
RUN curl -sSL https://sh.rustup.rs | sh
RUN source $HOME/.cargo/env

COPY zsign/tmp/libzsign.so /usr/lib/libzsign.so
COPY zsign/tmp/libzsign.so ./zsign/tmp/libzsign.so

COPY src .
COPY Cargo.toml .
COPY build.rs .
RUN cargo build --release
