FROM paritytech/ci-linux:production as build

RUN rustup default stable
RUN rustup toolchain uninstall nightly && rustup toolchain install nightly
RUN rustup update nightly

RUN rustup toolchain uninstall stable && rustup toolchain install stable
RUN rustup update stable
RUN rustup target add wasm32-unknown-unknown --toolchain stable

RUN rustup component add rust-src --toolchain stable
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

WORKDIR /code

# Copy the project files
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Runtime Stage
FROM ubuntu:22.04

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /node

# Copy the compiled binary from the build stage
COPY --from=build /code/target/release/node-template .

# Expose necessary ports for the node
EXPOSE 9944

# Define the command to run the node (adjust as needed for your node)
CMD ["./node-template", "--dev", "--rpc-external", "--rpc-methods=Unsafe"]
