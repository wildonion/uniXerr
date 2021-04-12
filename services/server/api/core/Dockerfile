

FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin core
WORKDIR ./core
ADD ../../../../server/core/ ./
ENV HOST=0.0.0.0 \
    PORT=8945
RUN cargo build --bin core --release --target x86_64-unknown-linux-musl


FROM alpine:latest
ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=core

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/core/target/x86_64-unknown-linux-musl/release/core ${APP}/core

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./core"]
