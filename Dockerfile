# ------- 1. CHEF STAGE ------- #
FROM messense/rust-musl-cross:x86_64-musl AS chef

RUN cargo install cargo-binstall --locked
RUN cargo binstall cargo-chef --no-confirm
RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo install dioxus-cli --locked --target x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# ------- 2. PLANNER STAGE ------- #
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ------- 3. BUILDER STAGE ------- #
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc
RUN dx build --release --fullstack

# ------- 4. CLEANER STAGE ------- #
FROM gcr.io/distroless/cc-debian12

COPY --from=builder --chmod=755 /app/target/dx/dioxus-ui/release/web/server  /server
COPY --from=builder /app/target/dx/dioxus-ui/release/web/public               /public

ENV IP=0.0.0.0
ENV PORT=8080
EXPOSE 8080

CMD ["/server"]
