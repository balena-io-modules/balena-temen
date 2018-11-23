#!/usr/bin/env bash

source $HOME/.nvm/nvm.sh
nvm use 10

# TODO: investigate if there is way for `wasm-pack test` to use the isomorphic package built in the previous step
wasm-pack test --chrome --firefox --headless
# test unified package as used from node

cp -r pkg/unified tests/wasm/node/temen-unified
cd tests/wasm/node/
npm install
# just one test for now, to see if `require` keyword works, as in if the packaging was successful
npm run test-require
