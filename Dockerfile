FROM rust:slim as builder
WORKDIR /app

COPY . .

# Build the project
RUN cargo install perseus-cli
RUN perseus deploy

# The alpine runner
FROM ubuntu:latest as runner

# Copy resources
COPY --from=builder /app/pkg/server /app/server

EXPOSE 8080
CMD ["/app/server"]
