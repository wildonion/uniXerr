

FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin freewifi
WORKDIR ./freewifi
# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --bin freewifi --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/actix_web_docker_example*
RUN cargo build --bin freewifi --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8587

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/actix-web-docker-example/target/x86_64-unknown-linux-musl/release/actix-web-docker-example ${APP}/actix-web-docker-example

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./freewifi"]
