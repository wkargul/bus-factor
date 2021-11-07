FROM rust:1.56 AS builder
ARG REPO_NAME
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src
RUN rustup component add clippy
RUN cargo clippy --all -- -D warnings
RUN cargo test --no-fail-fast --release
RUN cargo build --release

FROM centos as runner
ARG REPO_NAME
COPY --from=builder /app/target/release/$REPO_NAME /usr/bin/$REPO_NAME
ENV APP_CMD=$REPO_NAME
RUN echo $REPO_NAME
CMD $APP_CMD