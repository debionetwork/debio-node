FROM debian:buster-slim@sha256:dde4952b8e695e22586bf9bbb197141ff531e0a1ebcc7028264303d035a88069 AS runner
LABEL network.debio.image.authors="devops@debio.network"
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates && \
  # apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete; \
  # Create user and set ownership and permissions as required
  useradd -m -u 1001 -U -s /bin/sh -d /home/debio debio && \
  # manage folder data
  mkdir -p /home/debio/.local/share && \
  mkdir /data && \
  chown -R debio:debio /data && \
  ln -s /data /home/debio/.local/share/debio
# Add binnary to docker image
COPY ./debio /usr/local/bin
# Set to a non-root built-in user
USER debio
# check if executable works in this container
RUN /usr/local/bin/debio --version
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/debio"]
