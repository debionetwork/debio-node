FROM ubuntu:20.04
LABEL network.debio.image.authors="debio.dev@blocksphere.id"
# Create user and set ownership and permissions as required
RUN useradd -m -u 1001 -U -s /bin/sh -d /home/debio debio && \
  # manage folder data
  mkdir -p /home/debio/.local/share && \
  mkdir /data && \
  chown -R debio:debio /data && \
  ln -s /data /home/debio/.local/share/debio
# Add binnary to docker image
COPY ./debio /usr/local/bin
# Set to a non-root built-in user
USER debio
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/debio"]
