#!/usr/bin/env bash

set -e

# load up the environment - makes the tools be added to the path etc
source $HOME/.cargo/env

if [ "$TRAVIS_BRANCH" = "master" ]; then
    # repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
    cargo login "$CARGO_API_TOKEN"
    cargo publish
fi
