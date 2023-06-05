#!/bin/bash
rustup toolchain install nightly --component rust-src
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash # Optional
cd ~
espup install
. $HOME/export-esp.sh
echo "make sure to run export-esp for setting env variables"
