#!/usr/bin/env bash

################################################################################
#
# @nazrhom - this script exists, because `wasm-pack` tool doesn't support
#            `wasm-pack build --target all`. It supports either
#            `--target browser` or `--target nodejs`, but unfortunately
#            not both (yet).
#
# When this issue https://github.com/rustwasm/wasm-pack/issues/313 will be fixed,
# this whole script will be replaced with just one command:
#
# * wasm-pack build --target all --out-dir "$PKG"
#
# This script builds NPM package for NodeJS, another one for browser and then
# these two packages are merged into one isomorphic package.
#
# This script is executed from the `scripts/deploy.sh` script.
#

set -e
set -o pipefail

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

source "${HOME}/.nvm/nvm.sh"
nvm use
source "${HOME}/.cargo/env"

echo "Setting rustup override for this project"
rustup override set $(cat rust-toolchain)

################################################################################
#
# @nazrhom - don't change following paths, they're used in tests!
#

# Temporary directory for NPM package builds
TARGET_DIR="target/npm"
# Browser specific NPM package
BROWSER_PKG_DIR="${TARGET_DIR}/pkg-browser"
# Node specific NPM package
NODE_PKG_DIR="${TARGET_DIR}/pkg-node"
# Final / isomorphic NPM package
PKG_DIR="${TARGET_DIR}/pkg"

if [ -d "${TARGET_DIR}" ]; then
    rm -rf "${TARGET_DIR}"
fi
mkdir -p "${TARGET_DIR}"

echo "Packing NodeJS NPM package..."
wasm-pack build --target nodejs  --out-dir "${NODE_PKG_DIR}"

echo "Packing browser NPM package..."
wasm-pack build --target browser --out-dir "${BROWSER_PKG_DIR}"

echo "Building isomorphic NPM package..."
cp -r "${BROWSER_PKG_DIR}" "${PKG_DIR}/"
PKG_NAME=$(jq -r .name "${PKG_DIR}/package.json" | sed 's/\-/_/g')
sed "s/require[\(]'\.\/${PKG_NAME}_bg/require\('\.\/${PKG_NAME}_wasm/" "${NODE_PKG_DIR}/${PKG_NAME}.js" \
    > "${PKG_DIR}/${PKG_NAME}_main.js"
sed "s/require[\(]'\.\/${PKG_NAME}/require\('\.\/${PKG_NAME}_main/" "${NODE_PKG_DIR}/${PKG_NAME}_bg.js" \
    > "${PKG_DIR}/${PKG_NAME}_wasm.js"
jq ".files += [\"${PKG_NAME}_wasm.js\"]" ${PKG_DIR}/package.json \
    | jq ".main = \"${PKG_NAME}_main.js\"" \
    > ${PKG_DIR}/temp.json
mv -v "${PKG_DIR}/temp.json" "${PKG_DIR}/package.json"
rm -rf "${NODE_PKG_DIR}"
rm -rf "${BROWSER_PKG_DIR}"
