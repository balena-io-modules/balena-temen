#!/usr/bin/env bash

set -e
set -o pipefail

################################################################################
#
# @nazrhom - following section installs & updates Rust. Rust version is
#            specified in the `rust-toolchain` (nothing invented here, common
#            stuff)
#
# ----> Applies to both types `rust-crate` & `rust-crate-wasm`
#
echo "Installing Rust toolchain..."
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $(cat rust-toolchain)
source "${HOME}/.cargo/env"
rustup component add clippy
rustup component add rustfmt

################################################################################
#
# @nazrhom - if there's no caching involved, you can remove following lines,
#            it just updates packages & tooling in case they were cached
#            and installation was skipped in the previous step
#
echo "Updating Rust toolchain..."
(test -x "${HOME}/.cargo/bin/cargo-install-update" || cargo install cargo-update)
cargo install-update -a


#--------------------------- another repo.org.type ----------------------------#


################################################################################
#
# @nazrhom - following section installs NVM & NodeJS & Rust WASM. Node version
#            is specified in the `.nvmrc` file (nothing invented here, common stuff)
#
# ----> Applies to `rust-crate-wasm` only
#
echo "Installing NVM & NodeJS..."
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
source "${HOME}/.nvm/nvm.sh"
nvm install
nvm use
echo "NodeJS version $(node --version)"

echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f
