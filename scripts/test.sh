#!/usr/bin/env bash

set -e
set -o pipefail

HERE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

source "${HOME}/.nvm/nvm.sh"
nvm use
source "${HOME}/.cargo/env"

echo "Setting rustup override for this project"
rustup override set $(cat rust-toolchain)

TESTS_DIRECTORY=tests

################################################################################
#
# @nazrhom - following section checks formatting, run linters & tests
#
# ----> Applies to both types `rust-crate` & `rust-crate-wasm`
#
echo "Checking Rust crate formatting..."
cargo fmt -- --check

echo "Linting Rust crate..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Testing Rust crate..."
cargo test

echo "Trying to package Rust crate..."
CARGO_PACKAGE_ARGS=''
if [ -z "${CI}" ]; then
    # Allow to test uncommitted changes locally
    CARGO_PACKAGE_ARGS='--allow-dirty'
fi
cargo package ${CARGO_PACKAGE_ARGS}

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
"${HERE}/build-wasm.sh"

echo "Testing browser NPM package..."
wasm-pack test --chrome --firefox --headless

if [ -d "node/tests" ]; then
    echo "Testing NodeJS NPM package..."
    cd node/tests
    npm install
    npm test
    cd "${HERE}"
else
    echo "Skipping NodeJS NPM package tests, folder node/tests not found"
fi

