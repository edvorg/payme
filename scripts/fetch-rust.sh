#! /bin/bash

set -e

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
rustup target add wasm32-unknown-unknown
