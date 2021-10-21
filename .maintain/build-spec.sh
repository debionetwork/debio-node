#!/bin/sh
cargo build --release
./target/release/debio-node build-spec --disable-default-bootnode --chain dev> debioSpec.json 
sha256sum debioSpec.json> SHA256SUMS