FROM clux/muslrust:latest as chef
RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --target x86_64-alpine-linux-musl
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo build --release --target x86_64-alpine-linux-musl
RUN mv ./target/release/x86_64-alpine-linux-musl/vidhan-site ./site

FROM alpine:latest AS css-builder
WORKDIR /app
ADD --chmod=755 https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 ./
COPY ./styles.input.css ./tailwind.config.js ./
COPY ./src ./src
RUN ./tailwindcss-linux-x64 build -i ./styles.input.css -o ./styles.css --minify

FROM scratch AS runtime
WORKDIR /app
COPY ./public ./public
COPY ./content ./content
COPY --from=css-builder /app/styles.css ./public/css/styles.css
COPY --from=builder /app/site /usr/local/bin/
ENV PUBLIC_DIR /app/public
ENV CONTENT_DIR /app/content
ENTRYPOINT ["/usr/local/bin/site"]