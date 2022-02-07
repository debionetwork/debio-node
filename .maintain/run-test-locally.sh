#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo "Test Code"
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

popd