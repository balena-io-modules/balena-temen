#!/usr/bin/env bash

set -e

echo "Publishing ..."

# load up the environment - makes the tools be added to the path etc
source $HOME/.cargo/env

echo "Authenticating to cargo..."
# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
cargo login "$CARGO_API_TOKEN"
echo "Publishing to crates.io..."
cargo publish

# repo.yml.type == rust-crate-wasm
echo "Authenticating to npm..."
echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > ~/.npmrc
echo "Publishing to npm..."
npm publish --access public pkg/unified

echo "Done publishing to crates.io and npm"
