FROM rust as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates gcc && apt-get clean
COPY --from=builder /usr/local/cargo/bin/tortuga /usr/local/bin/tortuga

CMD ["tortuga"]