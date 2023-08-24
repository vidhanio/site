FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY ./Cargo.toml ./Cargo.lock ./build.rs ./tailwind.config.js ./styles.input.css ./
COPY ./src ./src
COPY ./content ./content
COPY ./tree-sitter ./tree-sitter
ADD --chmod=755 \
    https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    /usr/local/bin/tailwindcss
RUN cargo build --release
RUN mv ./target/release/vidhan-site ./site

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY ./static ./static
COPY --from=builder /app/site /usr/local/bin/
ENV STATIC_DIR /app/static
ENTRYPOINT ["/usr/local/bin/site"]