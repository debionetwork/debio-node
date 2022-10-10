FROM paritytech/ci-linux:production@sha256:535c8a792a618f549832fa57e52c346d352021fa050ec82b30073810eb15369e AS chef
RUN cargo install cargo-chef --version 0.1.31
WORKDIR /app

FROM chef AS planner
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./primitives /app/primitives
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Rebuild the source code only when needed
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./primitives /app/primitives
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release

# Production image, copy all the files and run
FROM debian:buster-slim@sha256:dde4952b8e695e22586bf9bbb197141ff531e0a1ebcc7028264303d035a88069 AS runner
LABEL network.debio.image.authors="devops@debio.network"
# Create user and set ownership and permissions as required
RUN useradd -m -u 1001 -U -s /bin/sh -d /home/debio debio && \
  # manage folder data
  mkdir -p /home/debio/.local/share && \
  mkdir /data && \
  chown -R debio:debio /data && \
  ln -s /data /home/debio/.local/share/debio
# Copy from builder stage
COPY --from=builder /app/target/release/debio /usr/local/bin
# Set to a non-root built-in user
USER debio
# check if executable works in this container
RUN /usr/local/bin/debio --version
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/debio"]
