#!/usr/bin/env bash

set -e
set -o pipefail

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
#
# Install Rust basic toolchain
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain `cat rust-toolchain`
rustup component add clippy-preview
rustup component add rustfmt-preview
(test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
cargo install-update -a

# repo.yml.type = rust-crate-wasm (wasm-only)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f


