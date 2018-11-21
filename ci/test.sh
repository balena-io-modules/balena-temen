#!/usr/bin/env bash

set -e
set -o pipefail

# good to have if we cache builds
cargo clean

# linters
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings

# build and test debug version
cargo test

# test packaging but do not publish
cargo package
