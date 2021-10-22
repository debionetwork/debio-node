#!/bin/sh
cargo build --release
./target/release/debio-node build-spec --disable-default-bootnode --chain debio-genesis > ./node/res/main/debioSpec.json 
sha256sum ./node/res/main/debioSpec.json> ./node/res/main/SHA256SUMS