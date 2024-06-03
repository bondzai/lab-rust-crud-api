FROM rust:1.72.0-slim-bullseye

# View app name in Cargo.toml
ARG APP_NAME=lab-rust

WORKDIR /app

COPY . .
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/bin/server"]