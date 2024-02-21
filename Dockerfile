# Builder: Downloads dependencies, compiles the project,
# passes on the executable
FROM rust:alpine as build

WORKDIR /app
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release

# User: Get the executable and run it
FROM alpine:latest

WORKDIR /app
COPY --from=build --chmod=711 /app/target/release/minidash .
COPY --chmod=711 entrypoint.sh .
COPY data defaults

ENV RUST_LOG="minidash"
ENV CONFIG_FILE="/app/data/config.yml"
ENV TEMPLATE_FILE="/app/data/template.hbs"
ENV STATIC_PATH="/app/data/static"
ENV ADDRESS="0.0.0.0:3000"

EXPOSE 3000/tcp

ENTRYPOINT ["./entrypoint.sh"]
CMD ["./minidash"]