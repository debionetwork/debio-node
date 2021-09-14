FROM phusion/baseimage:0.11 as builder

ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /debio

COPY . /debio

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y pkg-config libssl-dev \
		git clang build-essential \
    curl cmake protobuf-compiler

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN	make init && \
	make build

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11

# show backtraces
ENV RUST_BACKTRACE 1

ENV DEBIAN_FRONTEND=noninteractive

# install tools and dependencies
RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y \
		libssl1.1 \
		ca-certificates \
		curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
	useradd -m -u 1000 -U -s /bin/sh -d /debio debio

COPY --from=builder /debio/target/release/debio-node /usr/local/bin

USER debio

RUN /usr/local/bin/debio-node --version

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/debio-node"]
