FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN curl -L https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz \
    | tar xJf - \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/typst
RUN cargo build --release
RUN mv ./target/release/vidhan-site ./site

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/site /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/site"]