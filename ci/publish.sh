#!/usr/bin/env bash

set -e

if [ "$TRAVIS_BRANCH" = "master" ]; then
    # repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
    cargo login "$CARGO_API_TOKEN"
    cargo publish
fi
