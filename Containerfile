# Build stage
FROM rust:alpine3.21 as builder

WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev
COPY . .
# Build statically linked binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM scratch

# Copy the statically linked binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/zombuddy /

# Set the binary as the entrypoint
ENTRYPOINT ["/zombuddy"]
