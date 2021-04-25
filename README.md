<div align="center">
<img src="https://static.wixstatic.com/media/a1e4f4_3a53307330534938887d022a8978f5dd~mv2.png/v1/fill/w_100,h_100,al_c,q_85,usm_0.66_1.00_0.01/LogoGRAPH_Degenics_DNA_Testing_P.webp">
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
./target/release/debio-node --dev
```

Purge the development chain's state:

```bash
./target/release/debio-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/debio-node -lruntime=debug --dev
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/debio-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/debio-node --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/debio-node purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
