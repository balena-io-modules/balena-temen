#!/usr/bin/env bash

set -e
set -o pipefail

# load up the environment - makes the tools be added to the path etc
source $HOME/.cargo/env

# repo.yml.type = rust-* (rust-crate, rust-crate-wasm)
# these are linters
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings

# repo.yml.type = rust-* (rust-crate, rust-crate-wasm)
# this runs all the tests (except for WASM)
cargo test

# repo.yml.type = rust-* (rust-crate, rust-crate-wasm)
#
# Tries to create the package, but not publish it
cargo package

# repo.yml.type = rust-crate-wasm
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
"$DIR/build-wasm.sh"

# # repo.yml.type = rust-crate-wasm
#
# this runs tests from WASM
# this needs Chrome and Firefox installed, see .travis.yml
wasm-pack test --chrome --firefox --headless
# TODO add testing with node as well
