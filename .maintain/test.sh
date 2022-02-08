#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Check Code"
cargo +nightly check --all
echo "Start Testing Code"
for entry in `ls pallets`; do
  pushd .

  # The following line ensure we run from the project root
  PROJECT_ROOT=`git rev-parse --show-toplevel`
  cd $PROJECT_ROOT
  cd "pallets/${entry}"

  if [ -d "tests" ]; then
    # Take action if "tests" exists. #
    cd "tests"
  fi

  cargo +nightly test

  popd
done
echo "Check Lint"
cargo +nightly clippy --all -- -D warnings
echo "Check Format"
cargo +nightly fmt --all -- --check

popd
