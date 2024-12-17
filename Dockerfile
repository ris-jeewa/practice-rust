# Build Stage
FROM rust:1.82.0 AS builder

# Set working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy project files
COPY . .

# Build main project
RUN cargo build --release

# Production Stage
FROM debian:bookworm-slim

# Set the working directory in the final image
WORKDIR /app

ENV DATABASE_URL=postgres://postgres:root@172.31.16.3:5432/project2_db
ENV DATABASE_SCHEMA=public
ENV RUST_BACKTRACE=1

# Copy built binaries from builder stage
COPY --from=builder /app/target/release/practice-rust .

# Expose port 
EXPOSE 8080

# Run the application
CMD ["./practice-rust"]