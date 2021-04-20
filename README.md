<div align="center">
<img src="https://static.wixstatic.com/media/a1e4f4_3a53307330534938887d022a8978f5dd~mv2.png/v1/fill/w_100,h_100,al_c,q_85,usm_0.66_1.00_0.01/LogoGRAPH_Degenics_DNA_Testing_P.webp">
</div>

<div align="Center">
<h1> Debio Substrate Node</h1>
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


This project is still a work in progress

## Getting Started

Follow these steps to get started with the Node Template :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/debio-node -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

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

### Multi-Node Local Testnet

TODO

### Pallets

TODO

