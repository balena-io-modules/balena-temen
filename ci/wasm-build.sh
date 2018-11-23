#!/usr/bin/env bash

set -e
set -o pipefail

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

OUTPUT_DIR="pkg"
BROWSER_OUTPUT_DIR="$OUTPUT_DIR/browser"
NODE_OUTPUT_DIR="$OUTPUT_DIR/node"
UNIFIED_OUTPUT_DIR="$OUTPUT_DIR/unified"

# Clean previous packages
if [ -d "$OUTPUT_DIR" ]; then
    rm -rf "$OUTPUT_DIR"
fi

mkdir -p "$OUTPUT_DIR"

# Build for both targets
wasm-pack build --target nodejs  --out-dir "$NODE_OUTPUT_DIR"
wasm-pack build --target browser --out-dir "$BROWSER_OUTPUT_DIR"

# start with browser package and extend it to also support node
cp -r "${BROWSER_OUTPUT_DIR}" "${UNIFIED_OUTPUT_DIR}/"

# Get the package name
PKG_NAME=$(jq -r .name "$UNIFIED_OUTPUT_DIR/package.json" | sed 's/\-/_/g')

# Merge nodejs & browser packages
#
# The reason for this is that wasm-pack doesn't support --target=all, but
# just either --target nodejs or --target browser. We build these packages
# separately and the merge them into one isomorphic package for both
# environments.
#
# See https://github.com/rustwasm/wasm-pack/issues/313
cp -v "${NODE_OUTPUT_DIR}/${PKG_NAME}.js" "${UNIFIED_OUTPUT_DIR}/${PKG_NAME}_main.js"
sed "s/require[\(]'\.\/${PKG_NAME}/require\('\.\/${PKG_NAME}_main/" "${NODE_OUTPUT_DIR}/${PKG_NAME}_bg.js" > "${UNIFIED_OUTPUT_DIR}/${PKG_NAME}_bg.js"
jq ".files += [\"${PKG_NAME}_bg.js\"]" ${UNIFIED_OUTPUT_DIR}/package.json \
    | jq ".main = \"${PKG_NAME}_main.js\"" > ${UNIFIED_OUTPUT_DIR}/temp.json
mv -v "${UNIFIED_OUTPUT_DIR}/temp.json" "${UNIFIED_OUTPUT_DIR}/package.json"
rm -rf "${NODE_OUTPUT_DIR}"
rm -rf "${BROWSER_OUTPUT_DIR}"

