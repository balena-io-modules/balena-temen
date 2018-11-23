#!/usr/bin/env bash

set -e

# load up the environment - makes the tools be added to the path etc
source $HOME/.cargo/env

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
cargo login "$CARGO_API_TOKEN"
# Uncomment for balenaCI, for now, we will do it manually
# cargo publish

# repo.yml.type == rust-crate-wasm
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
"$DIR/wasm-build.sh"
# NPM_USER, _PASS, _EMAIL as Travis CI variables
npm-cli-login
# Uncomment for balenaCI, for now, we will do it manually
# npm publish --access public pkg
