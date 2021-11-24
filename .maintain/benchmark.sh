#!/usr/bin/env bash

set -e

# if [ "$#" -ne 1 ]; then
# 	echo "Please provide pallet name"
# 	exit 1
# fi

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

# PALLET=$1

# ./target/release/debio benchmark \
#   --chain=dev \
#   --execution=wasm \
#   --wasm-execution=compiled \
#   --pallet="$PALLET" \
#   --extrinsic="*" \
#   --steps=20 \
#   --repeat=10 \
#   --heap-pages=4096 \
#   --raw \
#   --output="./runtime/src/weights/${PALLET/-/_}.rs"

# shellcheck disable=SC2086
./target/release/debio benchmark \
    --chain dev \
    --list |\
  tail -n+2 |\
  cut -d',' -f1 |\
  uniq | \
  grep -v frame_system > pallets_list

# For each pallet found in the previous command, run benches on each function
while read -r line; do
  pallet="$(echo "$line" | cut -d' ' -f1)";
  echo "Pallet: $pallet";
# shellcheck disable=SC2086
done < pallets_list
rm pallets_list

popd
