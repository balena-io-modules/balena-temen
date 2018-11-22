#!/usr/bin/env bash

set -e
set -o pipefail

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
#
# Install Rust in a version specified in the `rust-toolchain` file
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain `cat rust-toolchain`
rustup component add clippy-preview
rustup component add rustfmt-preview

# repo.yml.type = rust-crate-wasm (wasm-only)

# update all tools installed using cargo
(test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
cargo install-update -a
# install wasm-pack and its dependencies, a tool that builds node packages from rust crates
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f


