#!/bin/sh
cargo build --release
./target/release/debio-node build-spec --disable-default-bootnode --chain dev > node/res/dev/debioDevSpec.json 
sha256sum node/res/dev/debioDevSpec.json> node/res/dev/SHA256SUMS