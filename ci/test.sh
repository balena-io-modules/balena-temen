#!/usr/bin/env bash

set -e
set -o pipefail

# repo.yml.type = rust-* (rust-crate, rust-crate-wasm)
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings

# repo.yml.type = rust-* (rust-crate, rust-crate-wasm)
cargo test

# repo.yml.type = rust-* (rust-crate, rust-crate-wasm)
#
# Tries to create the package, but not publish it
cargo package
