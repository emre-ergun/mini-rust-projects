FROM messense/rust-musl-cross:x86_64-musl as builder

WORKDIR /rust_docker_app

# Copy the source code
COPY . .
# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /rust_docker_app/target/x86_64-unknown-linux-musl/release/rust_docker_app .
ENTRYPOINT [ "/rust_docker_app" ]