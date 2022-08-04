FROM rust:1-buster

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# update cargo
RUN rustup self update
RUN rustup update stable

# Install production dependencies and build a release artifact.
RUN cargo build --release

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
ENTRYPOINT ["target/release/back"]