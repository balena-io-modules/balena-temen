#!/usr/bin/env bash

set -e
set -o pipefail

cargo test
cargo fmt -- --check

if [ ! "$CI" == "true" ]; then
    # When running locally, we have to clean the project, otherwise clippy
    # won't do nothing if the project was already compiled
    cargo clippy --all-targets --all-features -- -D warnings
fi
cargo clippy

if [ ! "$CI" == "true" ]; then
    # Allow uncommitted changes when running locally
    CARGO_PACKAGE_FLAGS="--allow-dirty"
fi

cargo package ${CARGO_PACKAGE_FLAGS}
