# Use the official Rust image from Docker Hub
FROM rust:latest

# Install MariaDB client as a replacement for MySQL client
RUN apt-get update && apt-get install -y mariadb-client-compat

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and src directory into the container
COPY Cargo.toml .
COPY src ./src

# Build the Rust project
RUN cargo build --release

# Run the executable
CMD ["./target/release/myapp"]
