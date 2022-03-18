FROM rust:alpine3.15 as build
WORKDIR /app
COPY . ./
RUN cargo build --release

FROM alpine:latest
COPY --from=build /app/target/release/killjoy_turret /usr/local/bin/killjoy_turret

COPY ./config.json /app/config.json
COPY ./entrypoint.sh /app/entrypoint.sh

ENV CONFIG_PATH=/app/config.json

ENTRYPOINT ["sh", "./entrypoint.sh"]