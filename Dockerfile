FROM phusion/baseimage:0.11 as builder

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release
WORKDIR /debio

COPY . /debio

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain install nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	rustup default stable && \
	cargo build "--$PROFILE"

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

COPY --from=builder /debio/target/$PROFILE/debio-node /usr/local/bin

USER debio

RUN /usr/local/bin/debio-node --version

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/debio-node"]
