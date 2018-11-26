#!/usr/bin/env bash

set -e

source "${HOME}/.cargo/env"
source "${HOME}/.nvm/nvm.sh"
nvm use


################################################################################
#
# @nazrhom - following section publishes Rust crate (package)
#
# CARGO_API_TOKEN env variable required. It does contain API token from crates.io
# (Fotis did create an account there and he has the token).
#
# ----> Applies to both types `rust-crate` & `rust-crate-wasm`
#
echo "Authenticating to crates.io..."
cargo login "${CARGO_API_TOKEN}"
echo "Publishing Rust crate..."
cargo publish


#--------------------------- another repo.org.type ----------------------------#


################################################################################
#
# @nazrhom - following section publishes isomorphic NPM package
#
# NPM_TOKEN env variable required. It does contain API token required for
# npmjs.com registry.
#
# ----> Applies to `rust-crate-wasm` only
#
echo "Authenticating to npmjs.org registry..."
echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > ~/.npmrc

# Build NPM package again, because ...
#
# a) script must succeed even if it is called without prior call to `ci/test.sh`
#    where the package is built,
# b) we don't know what happened in the meanwhile (between `ci/test.sh` & `ci/publish.sh`),
#    so, we build it again
ci/build-wasm.sh

echo "Publishing NPM package..."
npm publish --access public target/npm/pkg
