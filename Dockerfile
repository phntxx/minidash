# Builder: Downloads dependencies, compiles the project,
# passes on the executable
FROM rust:alpine as build

WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

# User: Get the executable and run it
FROM alpine:latest

RUN addgroup -g 1000 minidash
RUN adduser -D -s /bin/sh -u 1000 -G minidash minidash

WORKDIR /app
COPY --from=build --chmod=777 --chown=minidash:minidash /app/target/release/minidash-rust ./
USER minidash

ENV RUST_LOG="minidash_rust"
ENV CONFIG_FILE="/app/data/config.yml"
ENV TEMPLATE_FILE="/app/data/template.hbs"
ENV STATIC_PATH="/app/data/static"
ENV ADDRESS="0.0.0.0:3000"

ENTRYPOINT ["./minidash-rust"]