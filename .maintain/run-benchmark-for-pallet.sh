#!/usr/bin/env bash

for entry in `ls pallets`; do
  pushd .

  # The following line ensure we run from the project root
  PROJECT_ROOT=`git rev-parse --show-toplevel`
  cd $PROJECT_ROOT

  PALLET=$entry

  cargo +nightly run --release --locked --features=runtime-benchmarks -- benchmark \
    --chain=dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet="$PALLET" \
    --extrinsic="*" \
    --steps=20 \
    --repeat=10 \
    --heap-pages=4096 \
    --raw \
    --output="./pallets/${PALLET}/src/weights.rs" \
    --template="./.maintain/pallet-weight-template.hbs"

  popd
done
