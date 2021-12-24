FROM rust:alpine as builder
WORKDIR /usr/src/app
RUN apk add libc-dev
COPY . .
RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/tortuga /usr/local/bin/tortuga

ENTRYPOINT ["tortuga"]