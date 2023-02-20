<div align="center">
<img src="https://avatars.githubusercontent.com/u/76637246?s=200&v=4">
</div>

<div align="Center">
<h1>DeBio Node</h1>
<h2>Decentralized Sovereign Biomed</h2>
The Anonymous-First Platform for Medical and Bioinformatics Data.

<br>
<br>

[![Substrate](https://img.shields.io/badge/Substrate-3.0.0-brightgreen?logo=Parity%20Substrate)](https://substrate.io)
[![Medium](https://img.shields.io/badge/Medium-DeBio%20Network-brightgreen?logo=medium)](https://blog.debio.network)

</div>

---

DeBio Network is a decentralized anonymous-first platform for medical and bioinformatics data. It uses blockchain technology as the immutable transaction ledger to support its processes.

## Getting Started

Follow these steps to get started with our Blockchain Node

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/debug/debio \
--base-path .local \
--dev \
--alice \
--enable-offchain-indexing true
```

Purge the development chain's state:

```bash
./target/debug/debio \
purge-chain \
--base-path .local \
--dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/debug/debio \
--base-path .local \
--dev \
--alice \
--enable-offchain-indexing true \
-lruntime=debug
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./.maintain/docker/create-network.sh
```

```bash
./.maintain/docker/start-docker-compose.sh
```
