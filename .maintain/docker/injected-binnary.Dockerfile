FROM ubuntu:20.04@sha256:bffb6799d706144f263f4b91e1226745ffb5643ea0ea89c2f709208e8d70c999
LABEL network.debio.image.authors="dev@debio.network"
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates && \
  # apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete; \
    useradd -m -u 1001 -U -s /bin/sh -d /home/debio debio && \
  # manage folder data
    mkdir -p /home/debio/.local/share && \
    mkdir /data && \
    chown -R debio:debio /data && \
    ln -s /data /home/debio/.local/share/debio
# Add binnary to docker image
COPY --chown=debio ./debio /usr/local/bin
# Set to a non-root built-in user
USER debio
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/debio"]
