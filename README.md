<div align="center">
<img src="https://avatars.githubusercontent.com/u/76637246?s=200&v=4">
</div>

<div align="Center">
<h1> DeBio Node</h1>
<h2> Decentralized Sovereign Biomed </h2>
The Anonymous-First Platform for Medical and Bioinformatics Data.
<br>
Built on Substrate.

<br>
<br>

[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-brightgreen?logo=Parity%20Substrate)](https://substrate.dev/)
[![Medium](https://img.shields.io/badge/Medium-DeBio-brightgreen?logo=medium)](https://medium.com/@debionetwork.blog)
</div>

---

DeBio is a decentralized anonymous-first platform for medical and bioinformatics data. It uses blockchain technology as the immutable transaction ledger to support its processes.

DeBio blockchain nodes are built using Substrate, Parity's blockchain framework that allows for quick development of blockchains customizable by its pallets system.

- [Explanation about DeBio's pallets and functionalities](./docs/pallets.md)

## Getting Started

Follow these steps to get started with the Node

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/debug/debio \
--base-path .local \
--dev \
--alice
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
-lruntime=debug
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./.maintain/docker/start-docker-compose.sh
```
