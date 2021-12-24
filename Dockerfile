FROM rust as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM rust
COPY --from=builder /usr/local/cargo/bin/tortuga /usr/local/bin/tortuga

CMD ["tortuga"]