FROM alpine:3.14 as zsign
WORKDIR /zsign
RUN apk add --no-cache --virtual .build-deps p7zip curl bash zip unzip openssl-dev openssl-libs-static
RUN curl -fsSL https://xmake.io/shget.text | bash
COPY zsign .
RUN /root/.local/bin/xmake --root
RUN apk del .build-deps

#FROM zsign
#WORKDIR /app
#COPY . .
#RUN apk add --no-cache curl gcc
#RUN curl -sSL https://sh.rustup.rs | sh
#RUN source $HOME/.cargo/env
#RUN cargo build --release