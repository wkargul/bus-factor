FROM rust:1.56 AS builder
ARG REPO_NAME
WORKDIR /app
COPY Cargo.toml Cargo.toml
####pre-compile dependencies for local development
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/$REPO_NAME*
###
COPY src src
RUN rustup component add clippy
RUN cargo clippy --all -- -D warnings
RUN cargo build --release

FROM debian:buster-slim as runner
ARG REPO_NAME
COPY --from=builder /app/target/release/$REPO_NAME /usr/bin/$REPO_NAME
ENV APP_CMD=$REPO_NAME
RUN echo $REPO_NAME
CMD $APP_CMD