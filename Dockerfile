#FROM rust:latest as build
#RUN sudo apt-get install musl-tools
#WORKDIR /app
#COPY . ./
#RUN rustup target add x86_64-unknown-linux-musl
#RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
#COPY --from=build /app/target/x86_64-unknown-linux-musl/release/killjoy_turret /app/killjoy_turret
COPY ./target/x86_64-unknown-linux-musl/debug/killjoy_turret /usr/local/bin/killjoy_turret

COPY ./config.json /app/config.json
COPY ./entrypoint.sh /app/entrypoint.sh

ENV CONFIG_PATH=/app/config.json

ENTRYPOINT ["/usr/local/bin/killjoy_turret"]