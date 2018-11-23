#!/usr/bin/env bash

set -e

# load up the environment - makes the tools be added to the path etc
source $HOME/.cargo/env

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
cargo login "$CARGO_API_TOKEN"
cargo publish --dry-run

# repo.yml.type == rust-crate-wasm
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
"$DIR/wasm-build.sh"
npm-cli login
npm publish --access public pkg --dry-run
