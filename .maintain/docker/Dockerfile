FROM paritytech/ci-linux:9575dfcd-20210729 AS chef
RUN cargo install cargo-chef --version 0.1.31
WORKDIR /app

FROM chef AS planner
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./traits /app/traits
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Rebuild the source code only when needed
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./traits /app/traits
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release

# Production image, copy all the files and run
FROM debian:buster-20210816-slim AS runner
LABEL network.debio.image.authors="debio.dev@blocksphere.id"
# Create user and set ownership and permissions as required
RUN useradd -m -u 1001 -U -s /bin/sh -d /home/debio debio && \
  # manage folder data
  mkdir -p /home/debio/.local/share && \
  mkdir /data && \
  chown -R debio:debio /data && \
  ln -s /data /home/debio/.local/share/debio
# Copy from builder stage
COPY --from=builder /app/target/release/debio-node /usr/local/bin
# Set to a non-root built-in user
USER debio
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/debio-node"]
