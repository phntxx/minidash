# Builder: Downloads dependencies, compiles the project,
# passes on the executable
FROM rust:latest as build

WORKDIR /app
COPY . .
RUN cargo build --release

# User: Get the executable and run it
FROM alpine:latest

RUN addgroup -g 1000 minidash
RUN adduser -D -s /bin/sh -u 1000 -G minidash minidash

WORKDIR /app
COPY --from=build --chmod=777 --chown=minidash:minidash /app/target/release/minidash ./
COPY data defaults
USER minidash

ENV RUST_LOG="minidash"
ENV CONFIG_FILE="/app/data/config.yml"
ENV TEMPLATE_FILE="/app/data/template.hbs"
ENV STATIC_PATH="/app/data/static"
ENV ADDRESS="0.0.0.0:3000"

ENTRYPOINT ["./entrypoint.sh"]
