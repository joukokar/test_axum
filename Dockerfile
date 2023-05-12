FROM rust:1.69.0-bullseye as builder

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

WORKDIR /srv

COPY --from=builder /usr/local/cargo/bin/test_axum /srv/test_axum

CMD ["/srv/test_axum"]
