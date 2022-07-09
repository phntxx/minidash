# Builder: Downloads dependencies, compiles the project,
# passes on the executable
FROM rust:latest as build

WORKDIR /app
COPY . .
RUN cargo build --release

# User: Get the executable and run it
FROM alpine:latest

WORKDIR /app
COPY --from=build --chmod=700 /app/target/release/minidash ./
COPY data defaults

ENV RUST_LOG="minidash"
ENV CONFIG_FILE="/app/data/config.yml"
ENV TEMPLATE_FILE="/app/data/template.hbs"
ENV STATIC_PATH="/app/data/static"
ENV ADDRESS="0.0.0.0:3000"

ENTRYPOINT ["./entrypoint.sh"]
