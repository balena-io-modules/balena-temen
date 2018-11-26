#!/usr/bin/env bash

set -e
set -o pipefail

################################################################################
#
# @nazrhom - following section checks formatting, run linters & tests
#
# ----> Applies to both types `rust-crate` & `rust-crate-wasm`
#
source "${HOME}/.cargo/env"

echo "Checking Rust crate formatting..."
cargo fmt -- --check

echo "Linting Rust crate..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Testing Rust crate..."
cargo test

echo "Trying to package Rust crate..."
cargo package


#--------------------------- another repo.org.type ----------------------------#


################################################################################
#
# @nazrhom - following section tests NPM package
#
# ----> Applies to `rust-crate-wasm` only
#

# @nazrhom - check comments inside this script as it will be replaced by simple
#            `wasm-pack build --target all` in the future, details are inside
#
# Tests require to have Firefox & Chrome installed.
#
ci/build-wasm.sh

source "${HOME}/.nvm/nvm.sh"
nvm use
echo "NodeJS version $(node --version)"

echo "Testing browser NPM package..."
wasm-pack test --chrome --firefox --headless

if [ -d "node/tests" ]; then
    echo "Testing NodeJS NPM package..."
    DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
    cd node/tests
    npm install
    npm test
    cd "${DIR}"
else
    echo "Skipping NodeJS NPM package tests, folder node/tests not found"
fi
