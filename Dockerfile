# Install deps
FROM clux/muslrust:stable as perseus_cli
ARG CARGO_BUILD_TARGET=x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install perseus-cli --locked
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM perseus_cli AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM perseus_cli as builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build the project
RUN perseus deploy

# The alpine runner
FROM alpine:latest as runner

# Copy resources
COPY --from=builder /app/pkg /app/
RUN ls /app/server

EXPOSE 8080
CMD ["/app/server"]
