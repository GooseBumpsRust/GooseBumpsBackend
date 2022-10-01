FROM rust:1.61.0-buster AS builder
WORKDIR /usr/src/

RUN USER=root cargo new --lib goose_bumps_backend
WORKDIR /usr/src/goose_bumps_backend
COPY Cargo.toml ./
RUN echo "fn main() {}" > src/bin.rs
RUN cargo build --release
RUN rm src/*.rs
COPY src ./src
RUN touch src/lib.rs
RUN touch src/bin.rs
RUN cargo build --release

FROM rust:1.61.0-slim-buster

COPY --from=builder /usr/src/goose_bumps_backend/target/release/goose_bumps_backend /bin
USER 1000
COPY Rocket.toml ./Rocket.toml
CMD [ "goose_bumps_backend" ]
