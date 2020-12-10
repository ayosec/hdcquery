FROM rust:1.48-alpine AS builder

RUN apk add --no-cache libc-dev openssl-dev

COPY . /src
WORKDIR /src

RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release
RUN strip target/release/hdcquery



FROM alpine

RUN apk add --no-cache libgcc

COPY --from=builder /src/target/release/hdcquery /usr/local/bin/

USER nobody

ENTRYPOINT [ "/usr/local/bin/hdcquery" ]
