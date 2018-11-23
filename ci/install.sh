#!/usr/bin/env bash

set -e
set -o pipefail

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
#
# Install Rust in a version specified in the `rust-toolchain` file
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain `cat rust-toolchain`
source $HOME/.cargo/env
rustup component add clippy-preview
rustup component add rustfmt-preview

# update all tools installed using cargo - useful if caching of builds is enabled
(test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
cargo install-update -a

# repo.yml.type = rust-crate-wasm
#
# control version of node on the build machine:
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
source $HOME/.nvm/nvm.sh
nvm install 10
nvm use 10
# install wasm-pack and its dependencies, a tool that builds node packages from rust crates
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f
