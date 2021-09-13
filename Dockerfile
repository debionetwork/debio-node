FROM phusion/baseimage:0.11 as builder

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release

WORKDIR /debio

COPY . /debio

# Update default packages
RUN apt-get -qq update

# Get Ubuntu packages
RUN apt-get install -y -q \
    build-essential \
    curl \
		cmake \
		protobuf-compiler

# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

RUN make init

RUN cargo update -p parity-db

RUN make build

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
