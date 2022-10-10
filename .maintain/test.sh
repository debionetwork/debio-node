#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Check Lint"
cargo +nightly clippy --workspace -- -D warnings
echo "Check Format"
cargo +nightly fmt --all --check
echo "Start Testing"
cargo +nightly test --workspace

popd
