#!/usr/bin/env bash

set -e
set -o pipefail

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
#
# Install Rust basic toolchain
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain `cat rust-toolchain`
rustup component add clippy-preview
rustup component add rustfmt-preview

# repo.yml.type = rust-crate-wasm (wasm-only)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
