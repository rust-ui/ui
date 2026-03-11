# syntax=docker/dockerfile:1

# ------- 1. CHEF STAGE ------- #
FROM messense/rust-musl-cross:x86_64-musl as chef

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-chef
RUN cargo install --locked cargo-leptos
WORKDIR /app


# ------- 2. PLANNER STAGE ------- #
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# ------- 3. BUILDER STAGE ------- #
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
# Disable sccache as it's not installed in the container
ENV RUSTC_WRAPPER=""

# Path dependencies must exist before cargo chef cook can resolve them
COPY crates/tw_merge crates/tw_merge
COPY crates/tw_merge_variants crates/tw_merge_variants
COPY crates/_markdown_crate crates/_markdown_crate

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
RUN npm install

ENV LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl
RUN cargo leptos build --release


# ------- 4. CLEANER STAGE ------- #
# Using distroless instead of scratch to have a writable /tmp directory
# This is needed for SQLite file-based storage (bug_reports.db)
FROM gcr.io/distroless/static-debian12

COPY --from=builder --chmod=755 /app/target/x86_64-unknown-linux-musl/release/server /server
COPY --from=builder /app/target/site/ /site
# THIS ONE IS NEEDED FOR LEPTOS_MD
COPY --from=builder --chmod=444 /app/public /public

WORKDIR /

# Build args for secrets
ARG RESEND_TOKEN
ARG RESEND_AUDIENCE_ID
ARG RUSTIFY_API_URL
ARG BUG_REPORTS_API_KEY

ENV RUST_BACKTRACE=1
ENV LEPTOS_OUTPUT_NAME="deploy_rust_ui"
ENV LEPTOS_SITE_ROOT="/site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_ASSETS_DIR="public"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

# Resend environment variables
ENV RESEND_TOKEN=${RESEND_TOKEN}
ENV RESEND_AUDIENCE_ID=${RESEND_AUDIENCE_ID}

# Bug reports environment variables
ENV RUSTIFY_API_URL=${RUSTIFY_API_URL}
ENV BUG_REPORTS_API_KEY=${BUG_REPORTS_API_KEY}

EXPOSE 3000

CMD [ "./server" ]
