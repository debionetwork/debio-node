FROM ubuntu:20.04

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates \
		curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
	useradd -m -u 1000 -U -s /bin/sh -d /debio debio

COPY ./target/release/debio-node /usr/local/bin

USER debio

RUN /usr/local/bin/debio-node --version

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/debio-node"]
